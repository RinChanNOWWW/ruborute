use std::path::PathBuf;

use crate::{Error, Result};

use super::{
    music::{self, MusicStore},
    record::RecordStore,
};

pub struct FullRecord {
    music_id: u16,
    music_name: String,
    score: u32,
    difficulty: music::Difficulty,
    level: u8,
}

impl FullRecord {
    pub fn get_music_id(&self) -> u16 {
        self.music_id
    }
    pub fn get_music_name(&self) -> &str {
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

pub struct DataStore {
    record_store: RecordStore,
    music_store: MusicStore,
}

impl DataStore {
    pub fn open(
        user: String,
        record_path: impl Into<PathBuf>,
        music_path: impl Into<PathBuf>,
    ) -> Result<Self> {
        let music_store = MusicStore::open(music_path)?;
        let record_store = RecordStore::open(user, record_path)?;

        Ok(DataStore {
            music_store,
            record_store,
        })
    }

    pub fn get_music_record(&self, music_id: u16) -> Result<Vec<FullRecord>> {
        let mut res = Vec::new();
        if let Some(music) = self.music_store.get_music(music_id) {
            let name = music.get_name();
            println!("get records of music: {}...", &name);
            if let Some(records) = self.record_store.get_record(music_id) {
                for rec in records.iter() {
                    res.push(FullRecord {
                        music_id: rec.music_id,
                        music_name: name.clone(),
                        score: rec.score,
                        difficulty: music::Difficulty::from(rec.music_type)
                            .inf_ver(music.get_inf_ver()),
                        level: music.get_level(rec.music_type),
                    })
                }
            }
        } else {
            return Err(Error::DoCmdError(String::from("music not found")));
        }
        Ok(res)
    }
}
