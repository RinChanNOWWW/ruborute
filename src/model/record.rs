use super::music::{self, Music};
use derive_getters::Getters;
use serde::Deserialize;
use std::fmt::Display;

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
            ClearType::Complete => write!(f, "NC"),
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

impl Volfoce {
    pub fn new(vf: u32) -> Self {
        Self(vf)
    }
    pub fn get_internal(&self) -> u32 {
        self.0
    }
}

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

#[derive(Clone, Copy, Debug, Getters)]
pub struct LevelStat {
    level: u8,
    /// S
    s_num: u16,
    /// Triple A Plus (AAA+)
    tap_num: u16,
    /// Triple A (AAA)
    ta_num: u16,
    /// Clear
    nc_num: u16,
    /// Hard Clear
    hc_num: u16,
    /// UC
    uc_num: u16,
    /// PUC
    puc_num: u16,
    /// played total number
    played: u16,
}

impl LevelStat {
    pub fn new(
        level: u8,
        s_num: u16,
        tap_num: u16,
        ta_num: u16,
        nc_num: u16,
        hc_num: u16,
        uc_num: u16,
        puc_num: u16,
        played: u16,
    ) -> Self {
        Self {
            level,
            s_num,
            tap_num,
            ta_num,
            nc_num,
            hc_num,
            uc_num,
            puc_num,
            played,
        }
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn incr_s_num(&mut self, add: u16) {
        self.s_num += add;
    }
    pub fn incr_tap_num(&mut self, add: u16) {
        self.tap_num += add;
    }
    pub fn incr_ta_num(&mut self, add: u16) {
        self.ta_num += add;
    }
    pub fn incr_nc_num(&mut self, add: u16) {
        self.nc_num += add;
    }
    pub fn incr_hc_num(&mut self, add: u16) {
        self.hc_num += add;
    }
    pub fn incr_uc_num(&mut self, add: u16) {
        self.uc_num += add;
    }
    pub fn incr_puc_num(&mut self, add: u16) {
        self.puc_num += add;
    }
}

impl std::ops::Add<LevelStat> for LevelStat {
    type Output = LevelStat;
    fn add(self, rhs: LevelStat) -> Self::Output {
        LevelStat {
            level: self.level,
            s_num: self.s_num + rhs.s_num,
            tap_num: self.tap_num + rhs.tap_num,
            ta_num: self.ta_num + rhs.ta_num,
            nc_num: self.nc_num + rhs.nc_num,
            hc_num: self.hc_num + rhs.hc_num,
            uc_num: self.uc_num + rhs.uc_num,
            puc_num: self.puc_num + rhs.puc_num,
            played: self.played + rhs.played,
        }
    }
}