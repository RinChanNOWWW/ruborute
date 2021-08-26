use std::rc::Rc;

use prettytable::*;

use crate::{storage::MusicRecordStore, Error, Result};

use super::Cmd;

pub struct CmdHelp {}

impl CmdHelp {
    pub fn new() -> Self {
        CmdHelp {}
    }
}

impl Cmd for CmdHelp {
    fn do_cmd(&self) -> Result<()> {
        ptable!(
            ["Command", "Usage", "Description"],
            ["help", "help", "show the help information"],
            ["get", "get <music-id>", "find music records by music id"]
        );
        Ok(())
    }
}

pub struct CmdGet {
    cmds: Vec<String>,
    store: Rc<MusicRecordStore>,
}

impl CmdGet {
    pub fn new(store: Rc<MusicRecordStore>, vec: Vec<String>) -> Self {
        CmdGet { store, cmds: vec }
    }
}

impl Cmd for CmdGet {
    fn do_cmd(&self) -> Result<()> {
        if self.cmds.len() != 2 {
            return Err(Error::DoCmdError(String::from("args unmatched.")));
        }
        let music_id = self.cmds[1]
            .as_str()
            .parse::<u16>()
            .map_err(|e| Error::DoCmdError(e.to_string()))?;
        let record = self.store.get_music_record(music_id);
        match record {
            Some(item) => {
                ptable!(["music id", "score"], [music_id, item.score]);
                Ok(())
            }
            None => Err(Error::DoCmdError(String::from("music record not found"))),
        }
    }
}
