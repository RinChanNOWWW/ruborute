use std::rc::Rc;

use prettytable::{cell, row, table};

use crate::{storage::DataStore, Error, Result};

use super::Cmd;

pub struct CmdRecord {
    store: Rc<DataStore>,
}

impl CmdRecord {
    pub fn new(store: Rc<DataStore>) -> Self {
        CmdRecord { store }
    }
}

impl Cmd for CmdRecord {
    fn name(&self) -> &str {
        "record"
    }
    fn usage(&self) -> &str {
        "record <music-id>"
    }
    fn description(&self) -> &str {
        "get music record by music id"
    }

    fn do_cmd(&self, args: &[String]) -> Result<()> {
        if args.len() != 1 {
            return Err(Error::DoCmdError(String::from("args unmatched.")));
        }
        let music_id = args[0]
            .as_str()
            .parse::<u16>()
            .map_err(|e| Error::DoCmdError(e.to_string()))?;
        let records = self.store.get_music_record(music_id)?;
        if records.len() > 0 {
            let mut tab = table!(["music id", "music name", "difficulty", "level", "score"]);
            for rec in records {
                tab.add_row(row![
                    rec.get_music_id(),
                    rec.get_music_name(),
                    rec.get_difficulty(),
                    rec.get_level(),
                    rec.get_score()
                ]);
            }
            tab.printstd();
        } else {
            println!("you have not played this music yet.")
        }
        Ok(())
    }
}
