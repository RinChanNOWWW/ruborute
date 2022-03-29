use crate::config::AsphyxiaConfig;
use crate::Result;
use clap::Parser;
use serde::Deserialize;

use super::BemaniutilsConfig;

#[derive(Parser, Debug, Deserialize)]
#[clap(author, version, about)]
#[serde(default)]
pub struct Config {
    #[clap(long, short = 'c', default_value = "config.toml")]
    pub config_file: String,

    // reading data from asyphyxia's savedata.db.
    #[clap(flatten)]
    pub asyphyxia: AsphyxiaConfig,

    // reading data from online bemaniutils server.
    #[clap(flatten)]
    pub bemaniutils: BemaniutilsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_file: "config.toml".to_string(),
            asyphyxia: AsphyxiaConfig::default(),
            bemaniutils: BemaniutilsConfig::default(),
        }
    }
}

impl Config {
    /// load configs from args.
    pub fn load_from_args() -> Self {
        Config::parse()
    }

    /// load configs from toml file
    pub fn load_from_file(file: &str) -> Result<Self> {
        let txt = std::fs::read_to_string(file)?;
        let cfg: Config = toml::from_str(txt.as_str())?;
        Ok(cfg)
    }
}
