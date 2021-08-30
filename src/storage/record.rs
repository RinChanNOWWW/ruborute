use crate::Result;
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, fs::File, io::BufReader, path::PathBuf};

use super::music::{self, Music, MusicStore};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Grade {
    None,
    D,
    C,
    B,
    A,
    APlus,
    AA,
    AAPlus,
    AAA,
    AAAPlus,
    S,
}

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Grade::None => write!(f, "No Grade"),
            Grade::D => write!(f, "D"),
            Grade::C => write!(f, "C"),
            Grade::B => write!(f, "B"),
            Grade::A => write!(f, "A"),
            Grade::APlus => write!(f, "A+"),
            Grade::AA => write!(f, "AA"),
            Grade::AAPlus => write!(f, "AA+"),
            Grade::AAA => write!(f, "AAA"),
            Grade::AAAPlus => write!(f, "AAA+"),
            Grade::S => write!(f, "S"),
        }
    }
}

impl From<u8> for Grade {
    fn from(g: u8) -> Self {
        match g {
            1 => Grade::D,
            2 => Grade::C,
            3 => Grade::B,
            4 => Grade::A,
            5 => Grade::APlus,
            6 => Grade::AA,
            7 => Grade::AAPlus,
            8 => Grade::AAA,
            9 => Grade::AAAPlus,
            10 => Grade::S,
            _ => Grade::None,
        }
    }
}

impl Grade {
    pub fn get_vf_coef(&self) -> u64 {
        match *self {
            Grade::D => 80,
            Grade::C => 82,
            Grade::B => 85,
            Grade::A => 88,
            Grade::APlus => 91,
            Grade::AA => 94,
            Grade::AAPlus => 97,
            Grade::AAA => 100,
            Grade::AAAPlus => 102,
            Grade::S => 105,
            _ => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ClearType {
    None,
    Played,
    Complete,
    HardComplete,
    UltimateChain,
    PerfectUltimateChain,
}

impl Display for ClearType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ClearType::None => write!(f, "No Play"),
            ClearType::Played => write!(f, "Crash"),
            ClearType::Complete => write!(f, "Complete"),
            ClearType::HardComplete => write!(f, "HC"),
            ClearType::UltimateChain => write!(f, "UC"),
            ClearType::PerfectUltimateChain => write!(f, "PUC"),
        }
    }
}

impl From<u8> for ClearType {
    fn from(t: u8) -> Self {
        match t {
            1 => ClearType::Played,
            2 => ClearType::Complete,
            3 => ClearType::HardComplete,
            4 => ClearType::UltimateChain,
            5 => ClearType::PerfectUltimateChain,
            _ => ClearType::None,
        }
    }
}

impl ClearType {
    pub fn get_vf_coef(&self) -> u64 {
        match *self {
            ClearType::Complete => 100,
            ClearType::HardComplete => 102,
            ClearType::UltimateChain => 105,
            ClearType::PerfectUltimateChain => 110,
            _ => 50,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(default)]
    collection: String,
    #[serde(rename = "mid", default)]
    music_id: u16,
    #[serde(rename = "type", default)]
    music_type: u8,
    #[serde(default)]
    score: u32,
    #[serde(rename = "clear", default)]
    clear_type: u8,
    #[serde(default)]
    grade: u8,
    #[serde(rename = "__refid", default)]
    user_id: String,
}

impl Record {
    pub fn get_collectoin_str(&self) -> &str {
        self.collection.as_str()
    }
    pub fn get_music_id(&self) -> u16 {
        self.music_id
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn get_user_id_str(&self) -> &str {
        self.user_id.as_str()
    }
    pub fn get_music_type(&self) -> u8 {
        self.music_type
    }
    pub fn get_grade(&self) -> u8 {
        self.grade
    }
    pub fn get_clear_type(&self) -> u8 {
        self.clear_type
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Volfoce(u32);

impl Display for Volfoce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = self.0 / 10_u32.pow(3);
        let d = self.0 % 10_u32.pow(3);
        if d < 10 {
            write!(f, "{}.00{}", i, d)
        } else if d < 100 {
            write!(f, "{}.0{}", i, d)
        } else {
            write!(f, "{}.{}", i, d)
        }
    }
}

/// compute the volforce for a single record
///
/// reference: http://bemaniwiki.com/index.php?SOUND%20VOLTEX%20EXCEED%20GEAR/VOLFORCE
pub fn compute_volforce(level: u8, score: u32, grade: Grade, clear: ClearType) -> Volfoce {
    Volfoce(
        (level as u64 * score as u64 * grade.get_vf_coef() * clear.get_vf_coef() / 10_u64.pow(8))
            as u32,
    )
}

#[derive(Debug)]
pub struct FullRecord {
    music_id: u16,
    music_name: String,
    difficulty: music::Difficulty,
    level: u8,
    score: u32,
    grade: Grade,
    clear_type: ClearType,
    volfoce: Volfoce,
}

impl Clone for FullRecord {
    fn clone(&self) -> Self {
        FullRecord {
            music_id: self.get_music_id(),
            music_name: String::from(self.get_music_name_str()),
            difficulty: self.get_difficulty(),
            level: self.get_level(),
            score: self.get_score(),
            grade: self.get_grade(),
            clear_type: self.get_clear_type(),
            volfoce: self.get_volforce(),
        }
    }
}

impl FullRecord {
    pub fn from_record_with_music(rec: &Record, mus: Option<&Music>) -> Self {
        let mut ful_rec = FullRecord {
            music_id: rec.get_music_id(),
            music_name: String::from("(NOT FOUND)"),
            difficulty: music::Difficulty::Unknown,
            level: 0,
            score: rec.get_score(),
            grade: Grade::from(rec.get_grade()),
            clear_type: ClearType::from(rec.get_clear_type()),
            volfoce: Volfoce::default(),
        };
        if let Some(m) = mus {
            ful_rec.music_name = m.get_name();
            ful_rec.difficulty =
                music::Difficulty::from(rec.get_music_type()).inf_ver(m.get_inf_ver());
            ful_rec.level = m.get_level(rec.get_music_type());
        }
        ful_rec.volfoce = compute_volforce(
            ful_rec.level,
            ful_rec.score,
            ful_rec.grade,
            ful_rec.clear_type,
        );
        ful_rec
    }

    pub fn get_music_id(&self) -> u16 {
        self.music_id
    }
    pub fn get_music_name_str(&self) -> &str {
        self.music_name.as_str()
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn get_difficulty(&self) -> music::Difficulty {
        self.difficulty
    }
    pub fn get_level(&self) -> u8 {
        self.level
    }
    pub fn get_clear_type(&self) -> ClearType {
        self.clear_type
    }
    pub fn get_grade(&self) -> Grade {
        self.grade
    }
    pub fn get_volforce(&self) -> Volfoce {
        self.volfoce
    }
}

/// MusicRecordStore is used to get sdvx music record from asphyxia db file.
pub struct RecordStore {
    /// music records of current user.
    /// Vec<Record> contained music records of different levels.
    records: HashMap<u16, Vec<FullRecord>>,
}

impl RecordStore {
    /// open db file and load all music data to memory
    pub fn open(user: String, path: impl Into<PathBuf>, music_store: &MusicStore) -> Result<Self> {
        let path = path.into();
        let mut records: HashMap<u16, Vec<FullRecord>> = HashMap::new();
        // load data
        let mut reader = BufReader::new(File::open(&path)?);
        let mut stream = serde_json::Deserializer::from_reader(&mut reader).into_iter::<Record>();
        while let Some(item) = stream.next() {
            match item {
                Ok(music_record) => {
                    if music_record.get_collectoin_str() == "music"
                        && music_record.get_user_id_str() == user
                    {
                        // let music = music_store.
                        let music = music_store.get_music(music_record.music_id);
                        let full_record = FullRecord::from_record_with_music(&music_record, music);
                        if let Some(rec) = records.get_mut(&full_record.get_music_id()) {
                            if !rec.iter().any(|r| r.level == full_record.get_level()) {
                                rec.push(full_record.clone());
                            }
                            rec.sort_by_key(|r| r.get_level());
                        } else {
                            records.insert(full_record.get_music_id(), vec![full_record.clone()]);
                        }
                    }
                }
                _ => {}
            }
        }
        println!("your play data has been loaded.");
        println!("you have {} records.", records.len());
        Ok(RecordStore { records })
    }

    /// get music record by music id
    pub fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<&FullRecord> {
        self.records
            .iter()
            .filter(|(id, _)| music_id.contains(&id))
            .map(|(_, rec)| rec)
            .collect::<Vec<&Vec<FullRecord>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&FullRecord>>()
    }

    /// get the top 50 vf records
    pub fn get_best50(&self) -> Vec<&FullRecord> {
        let mut records = self
            .records
            .iter()
            .map(|(_, rec)| rec)
            .collect::<Vec<&Vec<FullRecord>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&FullRecord>>();
        records.sort_by_key(|rec| rec.volfoce);
        let mut res = Vec::with_capacity(50);
        let mut count = 0;
        for &rec in records.iter().rev() {
            if count == 50 {
                break;
            }
            res.push(rec);
            count += 1;
        }
        res
    }

    /// compute the complete volforce
    pub fn compute_volforce(&self) -> Volfoce {
        let best50 = self.get_best50();
        let mut vf_sum = 0;
        for rec in best50.iter() {
            vf_sum += rec.volfoce.0;
        }
        Volfoce(vf_sum / 50)
    }
}
