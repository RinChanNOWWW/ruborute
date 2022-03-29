mod asphyxia;
mod bemaniutils;

use crate::model::record::{FullRecord, LevelStat, Volfoce};

pub use asphyxia::AsphyxiaDataSource;
pub use bemaniutils::BemaniutilsDataSource;

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
}
