use super::DataSource;
use crate::config::BemaniutilsConfig;
use crate::model::{FullRecord, LevelStat};
use crate::Result;

pub struct BemaniutilsDataSource {}

impl DataSource for BemaniutilsDataSource {
    /// Get records of music_ids
    fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<FullRecord> {
        vec![]
    }
    /// Get records by name. The implementation is probably fuzzy search.
    fn get_record_by_name(&self, name: String) -> Vec<FullRecord> {
        vec![]
    }
    /// Get best 50 records of current user.
    fn get_best50_records(&self) -> Vec<FullRecord> {
        vec![]
    }
    /// Show how many CLEARs and GRADEs dose the user have at each type at the level.
    /// If `level` is `None`, return all level stats.
    fn get_level_stat(&self, level: Option<u8>) -> Vec<LevelStat> {
        vec![]
    }
    /// Show how many musics dose the user have played at the level.
    fn get_level_count(&self, level: u8) -> usize {
        0
    }
}

impl BemaniutilsDataSource {
    pub fn open(conf: BemaniutilsConfig) -> Result<Self> {
        println!("{:?}", conf);
        println!("data loaded from Bemaniutils server database succeeded!");
        Ok(Self {})
    }
}
