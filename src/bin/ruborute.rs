use std::process::exit;

use ruborute::{Cmdline, Config};

fn main() {
    let mut cfg = Config::load_from_args();
    if !cfg.config_file.is_empty() {
        let config_file = cfg.config_file;
        cfg = Config::load_from_file(config_file.as_str()).unwrap();
        cfg.config_file = config_file;
    }
    let cmdline = Cmdline::new(cfg);
    match cmdline {
        Ok(mut cl) => {
            if let Err(e) = cl.run() {
                eprintln!("{}", e);
                exit(1)
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}
