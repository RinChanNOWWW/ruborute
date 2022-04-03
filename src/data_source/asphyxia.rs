use crate::config::AsphyxiaConfig;
use crate::data_source::DataSource;
use crate::model::record::*;
use crate::storage::{AsphyxiaRecord, MusicStore};
use crate::Result;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

pub struct AsphyxiaDataSource {
    record_store: RecordStore,
    music_store: MusicStore,
}

impl AsphyxiaDataSource {
    pub fn open(conf: AsphyxiaConfig) -> Result<Self> {
        let music_store = MusicStore::open(conf.music_path)?;
        let record_store = RecordStore::open(conf.refid, conf.record_path, &music_store)?;

        println!("data loaded from Asphyxia succeeded!");
        Ok(AsphyxiaDataSource {
            music_store,
            record_store,
        })
    }
}
impl DataSource for AsphyxiaDataSource {
    fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<FullRecord> {
        for &id in music_id.iter() {
            println!("Music {}: <{}>", id, self.music_store.get_music_name(id));
        }
        self.record_store.get_record_by_id(music_id)
    }

    fn get_record_by_name(&self, name: String) -> Vec<FullRecord> {
        // get id by fuzzy searching
        let ids = self.music_store.get_id_by_name(&name, true);
        self.record_store.get_record_by_id(ids)
    }

    fn get_best50_records(&self) -> Vec<FullRecord> {
        self.record_store.get_best50()
    }

    fn get_volforce(&self) -> Volfoce {
        self.record_store.compute_volforce()
    }
    fn get_level_stat(&self, level: Option<u8>) -> Vec<LevelStat> {
        self.record_store.get_level_stat(level)
    }

    fn get_level_count(&self, level: u8) -> usize {
        self.music_store.get_level_count(level)
    }
}

/// MusicRecordStore is used to get sdvx music record from asphyxia db file.
struct RecordStore {
    /// music records of current user.
    /// Vec<Record> contained music records of different levels.
    records: HashMap<u16, HashMap<u8, FullRecord>>,
}

impl RecordStore {
    /// open db file and load all music data to memory
    pub fn open(user: String, path: impl Into<PathBuf>, music_store: &MusicStore) -> Result<Self> {
        let path = path.into();
        let mut records: HashMap<u16, HashMap<u8, FullRecord>> = HashMap::new();
        // load data
        let mut reader = BufReader::new(File::open(&path)?);
        let mut stream =
            serde_json::Deserializer::from_reader(&mut reader).into_iter::<AsphyxiaRecord>();
        while let Some(item) = stream.next() {
            match item {
                Ok(music_record) => {
                    if music_record.get_collectoin_str() == "music"
                        && music_record.get_refid_str() == user
                    {
                        // let music = music_store.
                        let music = music_store.get_music_ref(music_record.get_music_id());
                        let full_record = music_record.to_full_record(music);
                        if let Some(rec) = records.get_mut(&full_record.get_music_id()) {
                            let level = full_record.get_level();
                            if !rec.contains_key(&level) {
                                rec.insert(level, full_record);
                            } else if let Some(r) = rec.get_mut(&level) {
                                // record the best record
                                if r.get_volforce() < full_record.get_volforce() {
                                    *r = full_record;
                                }
                            }
                        } else {
                            let mut m = HashMap::new();
                            let id = full_record.get_music_id();
                            m.insert(full_record.get_level(), full_record);
                            records.insert(id, m);
                        }
                    }
                }
                _ => {}
            }
        }
        println!("{} records loaded.", records.len());
        Ok(RecordStore { records })
    }

    /// get music record by music id
    pub fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<FullRecord> {
        self.records
            .iter()
            .filter(|(id, _)| music_id.contains(&id))
            .map(|(_, rec)| rec)
            .collect::<Vec<&HashMap<u8, FullRecord>>>()
            .into_iter()
            .map(|map| {
                map.values()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<FullRecord>>()
            })
            .flatten()
            .collect::<Vec<FullRecord>>()
    }

    /// get the top 50 vf records
    pub fn get_best50(&self) -> Vec<FullRecord> {
        let mut records = self
            .records
            .iter()
            .map(|(_, rec)| rec)
            .collect::<Vec<&HashMap<u8, FullRecord>>>()
            .into_iter()
            .map(|map| {
                map.values()
                    .into_iter()
                    .cloned()
                    .collect::<Vec<FullRecord>>()
            })
            .flatten()
            .collect::<Vec<FullRecord>>();
        records.sort_by_key(|rec| rec.get_volforce());
        records
            .iter()
            .rev()
            .take(50)
            .cloned()
            .collect::<Vec<FullRecord>>()
    }

    /// compute the complete volforce
    pub fn compute_volforce(&self) -> Volfoce {
        let best50 = self.get_best50();
        let mut vf_sum = 0;
        for rec in best50.iter() {
            vf_sum += rec.get_volforce().get_internal();
        }
        Volfoce::new(vf_sum / 50)
    }

    /// get clear and grade type of a level.
    /// when level is None, return all level stat.
    pub fn get_level_stat(&self, level: Option<u8>) -> Vec<LevelStat> {
        let mut level_stat: HashMap<u8, LevelStat> = HashMap::new();
        for r in self
            .records
            .iter()
            .map(|(_, rec)| rec)
            .collect::<Vec<&HashMap<u8, FullRecord>>>()
            .into_iter()
            .map(|map| map.values().collect::<Vec<&FullRecord>>())
            .flatten()
            .filter(|r| match level {
                Some(l) => r.get_level() == l,
                None => true,
            })
        {
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
}
