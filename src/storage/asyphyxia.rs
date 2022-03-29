use std::time::SystemTime;

use crate::model::{music::*, record::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsphyxiaRecordDate {
    #[serde(rename = "$$date", default)]
    date: u128,
}

/// One music reocrd is like(but in one line):
///
/// ```json
/// {
///     "collection":"music",
///     "mid":913,
///     "type":3,
///     "score":9554896,
///     "exscore":0,
///     "clear":2,
///     "grade":7,
///     "buttonRate":8,
///     "longRate":9,
///     "volRate":9,
///     "_id":"009SMt6YgLg33p8n",
///     "createdAt":{"$$date":1633772620910},
///     "updatedAt":{"$$date":1635585980942},
///     "__a":"sdvx@asphyxia",
///     "__s":"plugins_profile",
///     "__refid":"AB973E24894A6D58"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AsphyxiaRecord {
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
    refid: String,

    // unused for reading data
    #[serde(rename = "_id", default)]
    id: String,
    #[serde(rename = "buttonRate", default)]
    button_rate: u8,
    #[serde(rename = "longRate", default)]
    long_rate: u8,
    #[serde(rename = "volRate", default)]
    vol_rate: u8,
    #[serde(rename = "createdAt", default)]
    create_at: AsphyxiaRecordDate,
    #[serde(rename = "updateAt", default)]
    update_at: AsphyxiaRecordDate,
    #[serde(rename = "__a", default)]
    a: String,
    #[serde(rename = "__s", default)]
    s: String,
}

impl AsphyxiaRecord {
    pub fn new_sdvx_record(
        refid: String,
        music_id: u16,
        music_type: u8,
        score: u32,
        clear_type: u8,
        grade: u8,
        id: String,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self {
            collection: "music".to_string(),
            music_id,
            music_type,
            score,
            clear_type,
            grade,
            refid,
            id,
            button_rate: 0,
            long_rate: 0,
            vol_rate: 0,
            create_at: AsphyxiaRecordDate { date: now },
            update_at: AsphyxiaRecordDate { date: now },
            a: "sdvx@asphyxia".to_string(),
            s: "plugins_profile".to_string(),
        }
    }

    pub fn get_collectoin_str(&self) -> &str {
        self.collection.as_str()
    }
    pub fn get_music_id(&self) -> u16 {
        self.music_id
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn get_refid_str(&self) -> &str {
        self.refid.as_str()
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

    pub fn to_full_record(&self, mus: Option<&Music>) -> FullRecord {
        let mut ful_rec = FullRecord {
            music_id: self.get_music_id(),
            music_name: String::from("(NOT FOUND)"),
            difficulty: Difficulty::Unknown,
            level: 0,
            score: self.get_score(),
            grade: Grade::from(self.get_grade()),
            clear_type: ClearType::from(self.get_clear_type()),
            volfoce: Volfoce::default(),
        };
        if let Some(m) = mus {
            ful_rec.music_name = m.get_name();
            ful_rec.difficulty = Difficulty::from(self.get_music_type()).inf_ver(m.get_inf_ver());
            ful_rec.level = m.get_level(self.get_music_type());
        }
        ful_rec.volfoce = compute_volforce(
            ful_rec.level,
            ful_rec.score,
            ful_rec.grade,
            ful_rec.clear_type,
        );
        ful_rec
    }
}
