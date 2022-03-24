use clap::Args;
use serde::Deserialize;

#[derive(Debug, Args, Deserialize)]
pub struct BemaniutilsConfig {
    #[clap(
        long,
        default_value = "",
        help = "the bemaniutils server database ip address"
    )]
    pub db_address: String,

    #[clap(
        long,
        default_value = "3306",
        help = "the bemaniutils server database port"
    )]
    pub db_port: u16,

    #[clap(long, default_value = "bemani", help = "the database name")]
    pub db_name: String,

    #[clap(long, default_value = "root", help = "the username of the database")]
    pub db_user: String,

    #[clap(long, default_value = "", help = "the password of the database")]
    pub db_password: String,

    #[clap(long, default_value = "", help = "the user to query")]
    pub username: String,
}

impl Default for BemaniutilsConfig {
    fn default() -> Self {
        Self {
            db_address: "".to_string(),
            db_port: 3306,
            db_name: "bemani".to_string(),
            db_user: "root".to_string(),
            db_password: "".to_string(),
            username: "".to_string(),
        }
    }
}
