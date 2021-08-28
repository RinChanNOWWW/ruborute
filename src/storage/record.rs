use crate::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use super::music::{self, Music, MusicStore};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(default)]
    collection: String,
    #[serde(rename = "mid", default)]
    music_id: u16,
    #[serde(default)]
    score: u32,
    #[serde(rename = "__refid", default)]
    user_id: String,
    #[serde(rename = "type", default)]
    music_type: u8,
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
}

#[derive(Debug)]
pub struct FullRecord {
    music_id: u16,
    music_name: String,
    score: u32,
    difficulty: music::Difficulty,
    level: u8,
}

impl FullRecord {
    pub fn from_record_with_music(rec: &Record, mus: Option<&Music>) -> Self {
        let mut ful_rec = FullRecord {
            music_id: rec.get_music_id(),
            music_name: String::from("(NOT FOUND)"),
            score: rec.get_score(),
            difficulty: music::Difficulty::Unknown,
            level: 0,
        };
        if let Some(m) = mus {
            ful_rec.music_name = m.get_name();
            ful_rec.difficulty =
                music::Difficulty::from(rec.get_music_type()).inf_ver(m.get_inf_ver());
            ful_rec.level = m.get_level(rec.get_music_type());
        }
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
                            rec.push(full_record);
                            rec.sort_by_key(|r| r.get_level());
                        } else {
                            records.insert(full_record.get_music_id(), vec![full_record]);
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
}
