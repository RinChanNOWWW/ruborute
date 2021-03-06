use std::collections::HashMap;

use super::DataSource;
use crate::config::BemaniutilsConfig;
use crate::model::{music::*, record::*};
use crate::storage::MusicStore;
use crate::{errors, Result};
use mysql::prelude::*;
use mysql::*;
use rust_fuzzy_search::fuzzy_compare;
use serde::Deserialize;

pub struct BemaniutilsDataSource {
    records: Vec<FullRecord>,
    music_store: MusicStore,
}

impl DataSource for BemaniutilsDataSource {
    /// Get records of music_ids
    fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<FullRecord> {
        self.records
            .iter()
            .filter(|r| music_id.contains(&r.music_id))
            .cloned()
            .collect()
    }
    /// Get records by name. The implementation is probably fuzzy search.
    fn get_record_by_name(&self, name: String) -> Vec<FullRecord> {
        self.records
            .iter()
            .filter(|r| {
                name.to_lowercase() == r.music_name.to_lowercase()
                    || fuzzy_compare(&name.to_lowercase(), &r.music_name.to_lowercase()) > 0.5
            })
            .cloned()
            .collect()
    }
    /// Get best 50 records of current user.
    fn get_best50_records(&self) -> Vec<FullRecord> {
        self.records.iter().take(50).cloned().collect()
    }
    /// Show how many CLEARs and GRADEs dose the user have at each type at the level.
    /// If `level` is `None`, return all level stats.
    fn get_level_stat(&self, level: Option<u8>) -> Vec<LevelStat> {
        let mut level_stat: HashMap<u8, LevelStat> = HashMap::new();
        for r in self.records.iter().filter(|r| match level {
            Some(l) => r.get_level() == l,
            None => true,
        }) {
            let mut stat = LevelStat::new(r.get_level(), 0, 0, 0, 0, 0, 0, 0, 1);
            match r.get_clear_type() {
                ClearType::Complete => stat.incr_nc_num(1),
                ClearType::HardComplete => stat.incr_hc_num(1),
                ClearType::UltimateChain => stat.incr_uc_num(1),
                ClearType::PerfectUltimateChain => stat.incr_puc_num(1),
                _ => {}
            }
            match r.get_grade() {
                Grade::AAA => stat.incr_ta_num(1),
                Grade::AAAPlus => stat.incr_tap_num(1),
                Grade::S => stat.incr_s_num(1),
                _ => {}
            }
            if let Some(old_stat) = level_stat.get_mut(&r.get_level()) {
                *old_stat = *old_stat + stat;
            } else {
                level_stat.insert(r.get_level(), stat);
            }
        }
        let mut r = level_stat
            .iter()
            .map(|(_, &s)| s)
            .collect::<Vec<LevelStat>>();
        r.sort_by_key(|&s| s.get_level());
        r
    }

    fn get_level_count(&self, level: u8) -> usize {
        self.music_store.get_level_count(level)
    }
}

impl BemaniutilsDataSource {
    pub fn open(conf: BemaniutilsConfig) -> Result<Self> {
        // read all need data when open
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            conf.db_user, conf.db_password, conf.db_address, conf.db_port, conf.db_name
        );
        let pool = Pool::new(mysql::Opts::from_url(url.as_str()).unwrap())?;
        let mut conn = pool.get_conn()?;
        // get user id by username first
        let user_id: u16 = if let Some(id) =
            conn.exec_first("SELECT id FROM user WHERE username = ?", (conf.username,))?
        {
            id
        } else {
            return Err(errors::Error::OtherError(
                "bemanitutils: username not found".to_string(),
            ));
        };

        let full_records = get_records_from_db(&mut conn, user_id, conf.game_version)?;
        let music = get_music_from_db(&mut conn)?;

        println!("{} records loaded.", full_records.len());
        println!("data loaded from Bemaniutils server database succeeded!");
        Ok(Self {
            records: full_records.into_iter().rev().collect(),
            music_store: music,
        })
    }
}

impl BemaniutilsDataSource {
    pub fn get_records(&self) -> Vec<FullRecord> {
        self.records.clone()
    }
}

fn get_records_from_db(
    conn: &mut PooledConn,
    user_id: u16,
    version: u8,
) -> Result<Vec<FullRecord>> {
    // get records by user id
    #[derive(Debug, Deserialize)]
    struct DBRecords {
        songid: u16,
        name: String,
        chart: u8,
        points: u32,
        sdata: String,
        mdata: String,
    }

    let sql = "SELECT music.songid AS songid, music.name AS name, music.chart AS chart, score.points AS points, score.data AS sdata, music.data AS mdata \
    FROM score, music \
    WHERE score.userid = ? AND score.musicid = music.id AND music.game = 'sdvx' AND music.version = ?";
    let result: Vec<DBRecords> = conn.exec_map(
        sql,
        (user_id, version),
        |(songid, name, chart, points, sdata, mdata)| DBRecords {
            songid,
            name,
            chart,
            points,
            sdata,
            mdata,
        },
    )?;

    let mut full_records = result
        .into_iter()
        .map(|r| {
            #[derive(Debug, Deserialize)]
            struct Mdata {
                difficulty: u8,
            }
            #[derive(Debug, Deserialize)]
            struct SData {
                grade: u16,
                clear_type: u16,
            }
            let mdata: Mdata = serde_json::from_str(r.mdata.as_str()).unwrap();
            let sdata: SData = serde_json::from_str(r.sdata.as_str()).unwrap();
            let grade = Grade::from(sdata.grade);
            let clear_type = ClearType::from(sdata.clear_type);

            FullRecord {
                music_id: r.songid,
                music_name: r.name,
                difficulty: Difficulty::from(r.chart),
                level: mdata.difficulty,
                score: r.points,
                grade: grade,
                clear_type: clear_type,
                volfoce: compute_volforce(mdata.difficulty, r.points, grade, clear_type),
            }
        })
        .collect::<Vec<FullRecord>>();

    full_records.sort_by_key(|rec| rec.get_volforce());
    Ok(full_records)
}

fn get_music_from_db(conn: &mut PooledConn) -> Result<MusicStore> {
    #[derive(Debug, Deserialize)]
    struct DBMusic {
        songid: u16,
        chart: u8,
        name: String,
        data: String,
    }
    #[derive(Debug, Deserialize)]
    struct Mdata {
        difficulty: u8,
    }

    let sql = "SELECT songid, chart, name, data FROM music";
    let result: Vec<DBMusic> = conn.exec_map(sql, (), |(songid, chart, name, data)| DBMusic {
        songid,
        chart,
        name,
        data,
    })?;
    let mut music: HashMap<u16, Music> = HashMap::new();
    let mut name_id_map: HashMap<String, u16> = HashMap::new();

    for m in result.iter() {
        let mdata: Mdata = serde_json::from_str(m.data.as_str()).unwrap();
        if mdata.difficulty == 0 {
            continue;
        }
        if !name_id_map.contains_key(&m.name) {
            name_id_map.insert(m.name.clone(), m.songid);
            music.insert(m.songid, Music::default());
        }
        if let Some(mut ms) = music.get_mut(&m.songid) {
            match m.chart {
                0 => ms.difficulty.novice.level = mdata.difficulty,
                1 => ms.difficulty.advanced.level = mdata.difficulty,
                2 => ms.difficulty.exhaust.level = mdata.difficulty,
                3 => ms.difficulty.infinite.level = mdata.difficulty,
                _ => ms.difficulty.maximum.level = mdata.difficulty,
            }
        }
    }

    Ok(MusicStore { music, name_id_map })
}
