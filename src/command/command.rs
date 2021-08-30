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
        let records = if let Ok(music_id) = args[0].as_str().parse::<u16>() {
            // the first arg is u16, to get record by id
            self.store.get_record_by_id(vec![music_id])
        } else {
            // else, all the args remain are join to music name,
            // and get record by the name.
            let name = args.join(" ");
            self.store.get_record_by_name(name)
        };
        if records.len() > 0 {
            let mut tab = table!([
                "music id",
                "music name",
                "difficulty",
                "level",
                "score",
                "grade",
                "clear type",
                "volforce"
            ]);
            for rec in &records {
                tab.add_row(row![
                    rec.get_music_id(),
                    rec.get_music_name_str(),
                    rec.get_difficulty(),
                    rec.get_level(),
                    rec.get_score(),
                    rec.get_grade(),
                    rec.get_clear_type(),
                    rec.get_volforce(),
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

pub struct CmdBest50 {
    store: Rc<DataStore>,
}

impl CmdBest50 {
    pub fn new(store: Rc<DataStore>) -> Self {
        CmdBest50 { store }
    }
}

impl Cmd for CmdBest50 {
    fn name(&self) -> &str {
        "best50"
    }
    fn usage(&self) -> &str {
        "best50"
    }
    fn description(&self) -> &str {
        "get the best 50 records in volforce order."
    }

    fn do_cmd(&self, _: &[String]) -> Result<()> {
        let records = self.store.get_best50_records();
        if records.len() > 0 {
            let mut tab = table!([
                "rank",
                "music id",
                "music name",
                "difficulty",
                "level",
                "score",
                "grade",
                "clear type",
                "volforce"
            ]);
            for (i, rec) in records.iter().enumerate() {
                tab.add_row(row![
                    format!("#{}", i + 1),
                    rec.get_music_id(),
                    rec.get_music_name_str(),
                    rec.get_difficulty(),
                    rec.get_level(),
                    rec.get_score(),
                    rec.get_grade(),
                    rec.get_clear_type(),
                    rec.get_volforce(),
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

pub struct CmdVolforce {
    store: Rc<DataStore>,
}

impl CmdVolforce {
    pub fn new(store: Rc<DataStore>) -> Self {
        CmdVolforce { store }
    }
}

impl Cmd for CmdVolforce {
    fn name(&self) -> &str {
        "vf"
    }
    fn usage(&self) -> &str {
        "vf"
    }
    fn description(&self) -> &str {
        "compute and print your volforce."
    }

    fn do_cmd(&self, _: &[String]) -> Result<()> {
        let vf = self.store.get_volforce();
        println!("Your Volforce: {}", vf);
        Ok(())
    }
}
