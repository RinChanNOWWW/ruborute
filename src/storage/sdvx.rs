use crate::model::music::Music;
use crate::Result;
use quick_xml;
use rust_fuzzy_search::fuzzy_compare;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Deserialize, PartialEq)]
struct Mdb {
    music: Vec<Music>,
}

pub struct MusicStore {
    pub music: HashMap<u16, Music>,
    pub name_id_map: HashMap<String, u16>,
}

impl MusicStore {
    fn from_mdb(mdb: Mdb) -> Self {
        let mut music: HashMap<u16, Music> = HashMap::new();
        let mut name_id_map: HashMap<String, u16> = HashMap::new();
        for m in mdb.music.iter() {
            music.insert(m.get_id(), m.clone());
            name_id_map.insert(m.get_name(), m.get_id());
        }
        MusicStore { music, name_id_map }
    }
}

impl MusicStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let mdb: Mdb = quick_xml::de::from_reader(BufReader::new(File::open(path.into())?))?;
        Ok(MusicStore::from_mdb(mdb))
    }

    pub fn get_music_ref(&self, music_id: u16) -> Option<&Music> {
        self.music.get(&music_id)
    }

    pub fn get_music_name(&self, music_id: u16) -> String {
        if let Some(name) = self.get_music_ref(music_id).map(|m| m.get_name()) {
            name
        } else {
            String::from("(NOT FOUND)")
        }
    }

    pub fn get_id_by_name(&self, name: &String, fuzzy: bool) -> Vec<u16> {
        self.name_id_map
            .iter()
            .filter(|(n, _)| {
                if fuzzy {
                    name.to_lowercase() == n.to_lowercase()
                        || fuzzy_compare(&name.to_lowercase(), &n.to_lowercase()) > 0.5
                } else {
                    name.as_str() == n.as_str()
                }
            })
            .map(|(_, &id)| id)
            .collect::<Vec<u16>>()
    }

    pub fn get_level_count(&self, level: u8) -> usize {
        self.music
            .iter()
            .filter(|(_, m)| m.has_level(level))
            .count()
    }
}
