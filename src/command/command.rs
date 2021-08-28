use std::rc::Rc;

use prettytable::{cell, row, table};

use crate::{storage::DataStore, Error, Result};

use super::Cmd;

/// `CmdRecord` is used to get gaming data from storage.
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
        "record <music-id | music-name>"
    }
    fn description(&self) -> &str {
        "get music record by the music id or name."
    }

    fn do_cmd(&self, args: &[String]) -> Result<()> {
        if args.len() < 1 {
            return Err(Error::DoCmdError(String::from("args unmatched.")));
        }
        let mut records = Vec::new();
        if let Ok(music_id) = args[0].as_str().parse::<u16>() {
            // the first arg is u16, to get record by id
            records = self.store.get_record_by_id(vec![music_id]);
        } else {
            // else, all the args remain are join to music name,
            // and get record by the name.
            let name = args.join(" ");
            records = self.store.get_record_by_name(name);
        }
        if records.len() > 0 {
            let mut tab = table!(["music id", "music name", "difficulty", "level", "score"]);
            for rec in &records {
                tab.add_row(row![
                    rec.get_music_id(),
                    rec.get_music_name_str(),
                    rec.get_difficulty(),
                    rec.get_level(),
                    rec.get_score()
                ]);
            }
            tab.printstd();
            println!("{} record(s) founded.", records.len());
        } else {
            println!("The music record not found.")
        }
        Ok(())
    }
}
