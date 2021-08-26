use std::rc::Rc;

use crate::{storage::MusicRecordStore, Error, Result};

use self::command::*;

mod command;

pub trait Cmd {
    fn do_cmd(&self) -> Result<()>;
}

pub fn new_command(store: Rc<MusicRecordStore>, vec: Vec<String>) -> Result<Box<dyn Cmd>> {
    match vec[0].as_str() {
        "help" => Ok(Box::new(CmdHelp::new())),
        "get" => Ok(Box::new(CmdGet::new(store, vec))),
        _ => Err(Error::OtherError(String::from("no such command"))),
    }
}
