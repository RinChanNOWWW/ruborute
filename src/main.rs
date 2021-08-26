use std::process::exit;

use asphyxia_rsdvx::{run_cmdline, Opt};
use clap::Clap;

fn main() {
    if let Err(e) = run_cmdline(Opt::parse()) {
        eprintln!("{}", e);
        exit(1)
    }
}
