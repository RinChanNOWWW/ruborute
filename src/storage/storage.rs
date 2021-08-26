use crate::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "mid")]
    pub music_id: u16,
    pub score: u32,
    #[serde(rename = "__refid")]
    pub user_id: String,
    // unused fileds
    // collection: String,
    // type: u8,
    // clear: u8,
    // grade: u32,
    // #[serde(rename = "buttonRate")]
    // button_rate: u16,
    // #[serde(rename = "longRate")]
    // long_rate: u16,
    // #[serde(rename = "_id")]
    // id: String,
    // #[serde(rename = "createdAt")]
    // created_at: HashMap<String, u64>,
    // #[serde(rename = "updatedAt")]
    // updated_at: HashMap<String, u64>,
    // __s: String,
}

/// MusicRecordStore is used to get sdvx music record from asphyxia db file.
pub struct MusicRecordStore {
    /// user id
    pub user: String,
    /// music records of current user. music_id -> Item
    records: HashMap<u16, Item>,
}

impl MusicRecordStore {
    /// open db file and load all music data to memory
    pub fn open(user: String, path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let mut records: HashMap<u16, Item> = HashMap::new();
        // load data
        let mut reader = BufReader::new(File::open(&path)?);
        let mut stream = serde_json::Deserializer::from_reader(&mut reader).into_iter::<Item>();
        while let Some(item) = stream.next() {
            match item {
                Ok(music_record) => {
                    if music_record.user_id == user {
                        let music_id = music_record.music_id;
                        let origin_record = records.get_mut(&music_id);
                        match origin_record {
                            Some(origin) => {
                                // save the max score
                                if origin.score < music_record.score {
                                    origin.score = music_record.score
                                }
                            }
                            None => {
                                records.insert(music_id, music_record);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(MusicRecordStore { user, records })
    }

    pub fn get_user(&self) -> &str {
        self.user.as_str()
    }

    /// get music record buy music id
    pub fn get_music_record(&self, music_id: u16) -> Option<&Item> {
        self.records.get(&music_id)
    }
}
