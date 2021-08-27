use crate::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(default)]
    collection: String,
    #[serde(rename = "mid", default)]
    pub music_id: u16,
    #[serde(default)]
    pub score: u32,
    #[serde(rename = "__refid", default)]
    pub user_id: String,
    #[serde(rename = "type", default)]
    pub music_type: u8,
}

/// MusicRecordStore is used to get sdvx music record from asphyxia db file.
pub struct RecordStore {
    /// user id
    pub user: String,
    /// music records of current user.
    /// Vec<Record> contained music records of different levels.
    records: HashMap<u16, Vec<Record>>,
}

impl RecordStore {
    /// open db file and load all music data to memory
    pub fn open(user: String, path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let mut records: HashMap<u16, Vec<Record>> = HashMap::new();
        // load data
        let mut reader = BufReader::new(File::open(&path)?);
        let mut stream = serde_json::Deserializer::from_reader(&mut reader).into_iter::<Record>();
        while let Some(item) = stream.next() {
            match item {
                Ok(music_record) => {
                    if music_record.collection.as_str() == "music" && music_record.user_id == user {
                        if let Some(rec) = records.get_mut(&music_record.music_id) {
                            rec.push(music_record);
                            rec.sort_by_key(|r| r.music_type);
                        } else {
                            records.insert(music_record.music_id, vec![music_record]);
                        }
                    }
                }
                _ => {}
            }
        }

        println!("your play data has been loaded.");
        println!("you have {} records.", records.len());
        Ok(RecordStore { user, records })
    }

    /// get music record buy music id
    pub fn get_record(&self, music_id: u16) -> Option<&Vec<Record>> {
        self.records.get(&music_id)
    }
}
