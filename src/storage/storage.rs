use std::path::PathBuf;

use crate::Result;

use super::{
    music::MusicStore,
    record::{FullRecord, RecordStore, Volfoce},
};

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
        let record_store = RecordStore::open(user, record_path, &music_store)?;

        Ok(DataStore {
            music_store,
            record_store,
        })
    }

    pub fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<&FullRecord> {
        for &id in music_id.iter() {
            println!("Music {}: <{}>", id, self.music_store.get_music_name(id));
        }
        self.record_store.get_record_by_id(music_id)
    }

    pub fn get_record_by_name(&self, name: String) -> Vec<&FullRecord> {
        // get id by fuzzy searching
        let ids = self.music_store.get_id_by_name(&name, true);
        self.record_store.get_record_by_id(ids)
    }

    pub fn get_best50_records(&self) -> Vec<&FullRecord> {
        self.record_store.get_best50()
    }

    pub fn get_volforce(&self) -> Volfoce {
        self.record_store.compute_volforce()
    }
}
