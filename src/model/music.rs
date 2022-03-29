use serde::Deserialize;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Unknown,
    Novice,
    Advanced,
    Exhaust,
    Infinite,
    Gravity,
    Heaven,
    Vivid,
    Maximum,
}

impl Difficulty {
    pub fn inf_ver(&self, inf_ver: u8) -> Self {
        if *self == Difficulty::Infinite {
            match inf_ver {
                3 => Difficulty::Gravity,
                4 => Difficulty::Heaven,
                5 => Difficulty::Vivid,
                _ => *self,
            }
        } else {
            *self
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Difficulty::Novice => write!(f, "NOV"),
            Difficulty::Advanced => write!(f, "ADV"),
            Difficulty::Exhaust => write!(f, "EXH"),
            Difficulty::Infinite => write!(f, "INF"),
            Difficulty::Gravity => write!(f, "GRV"),
            Difficulty::Heaven => write!(f, "HVN"),
            Difficulty::Vivid => write!(f, "VVD"),
            Difficulty::Maximum => write!(f, "MXM"),
            Difficulty::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl From<u8> for Difficulty {
    /// from music type to Difficulty
    fn from(d: u8) -> Self {
        match d {
            0 => Difficulty::Novice,
            1 => Difficulty::Advanced,
            2 => Difficulty::Exhaust,
            3 => Difficulty::Infinite,
            4 => Difficulty::Maximum,
            _ => Difficulty::Unknown,
        }
    }
}

impl Into<u8> for Difficulty {
    fn into(self) -> u8 {
        match self {
            Difficulty::Novice => 0,
            Difficulty::Advanced => 1,
            Difficulty::Exhaust => 2,
            Difficulty::Infinite => 3,
            Difficulty::Maximum => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
struct DiffInfo {
    #[serde(rename = "difnum")]
    level: u8,
}

impl Default for DiffInfo {
    fn default() -> Self {
        DiffInfo { level: 0 }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
struct MusicDiffculty {
    #[serde(default)]
    novice: DiffInfo,
    #[serde(default)]
    advanced: DiffInfo,
    #[serde(default)]
    exhaust: DiffInfo,
    #[serde(default)]
    infinite: DiffInfo,
    #[serde(default)]
    maximum: DiffInfo,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MusicInfo {
    #[serde(rename = "title_name")]
    name: String,
    inf_ver: u8,
}

impl Clone for MusicInfo {
    fn clone(&self) -> Self {
        MusicInfo {
            name: self.name.clone(),
            inf_ver: self.inf_ver,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Music {
    id: u16,
    info: MusicInfo,
    difficulty: MusicDiffculty,
}

impl Music {
    pub fn get_id(&self) -> u16 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.info.name.clone()
    }
    pub fn get_inf_ver(&self) -> u8 {
        self.info.inf_ver
    }
    pub fn get_level(&self, d: impl Into<Difficulty>) -> u8 {
        match d.into() {
            Difficulty::Novice => self.difficulty.novice.level,
            Difficulty::Advanced => self.difficulty.advanced.level,
            Difficulty::Exhaust => self.difficulty.exhaust.level,
            Difficulty::Maximum => self.difficulty.maximum.level,
            Difficulty::Unknown => 0,
            _ => self.difficulty.infinite.level,
        }
    }
}

impl Clone for Music {
    fn clone(&self) -> Self {
        Music {
            id: self.id,
            info: self.info.clone(),
            difficulty: self.difficulty,
        }
    }
}
