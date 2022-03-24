mod asphyxia;

use crate::model::{FullRecord, LevelStat, Volfoce};

pub use asphyxia::AsphyxiaDataStore;

pub trait DataSource {
    /// Get records of music_ids
    fn get_record_by_id(&self, music_id: Vec<u16>) -> Vec<FullRecord>;
    /// Get records by name. The implementation is probably fuzzy search.
    fn get_record_by_name(&self, name: String) -> Vec<FullRecord>;
    /// Get best 50 records of current user.
    fn get_best50_records(&self) -> Vec<FullRecord>;
    /// Get current user's vf
    fn get_volforce(&self) -> Volfoce {
        let best50 = self.get_best50_records();
        let vf_sum: u32 = best50.iter().map(|r| r.get_volforce().get_internal()).sum();
        Volfoce::new(vf_sum / 50)
    }
    /// Show how many CLEARs and GRADEs dose the user have at each type at the level.
    /// If `level` is `None`, return all level stats.
    fn get_level_stat(&self, level: Option<u8>) -> Vec<LevelStat>;
    /// Show how many musics dose the user have played at the level.
    fn get_level_count(&self, level: u8) -> usize;
}
