use std::process::exit;

use clap::Clap;
use ruborute::{Cmdline, Opt};

fn main() {
    let cmdline = Cmdline::new(Opt::parse());
    match cmdline {
        Ok(cl) => {
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
