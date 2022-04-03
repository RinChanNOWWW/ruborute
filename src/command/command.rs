use std::rc::Rc;

use prettytable::{cell, row, table};

use crate::{data_source::DataSource, Error, Result};

use super::Cmd;

/// `CmdRecord` is used to get gaming data from storage.
pub struct CmdRecord<T: DataSource> {
    store: Rc<T>,
}

impl<T: DataSource> CmdRecord<T> {
    pub fn new(store: Rc<T>) -> Self {
        CmdRecord { store }
    }
}

impl<T: DataSource> Cmd for CmdRecord<T> {
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
            return Err(Error::DoCmdError(String::from(
                "The music record not found.",
            )));
        }
        Ok(())
    }
}

pub struct CmdBest50<T: DataSource> {
    store: Rc<T>,
}

impl<T: DataSource> CmdBest50<T> {
    pub fn new(store: Rc<T>) -> Self {
        CmdBest50 { store }
    }
}

impl<T: DataSource> Cmd for CmdBest50<T> {
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
            return Err(Error::DoCmdError(String::from(
                "The music record not found.",
            )));
        }
        Ok(())
    }
}

pub struct CmdVolforce<T: DataSource> {
    store: Rc<T>,
}

impl<T: DataSource> CmdVolforce<T> {
    pub fn new(store: Rc<T>) -> Self {
        CmdVolforce { store }
    }
}

impl<T: DataSource> Cmd for CmdVolforce<T> {
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

pub struct CmdCount<T: DataSource> {
    store: Rc<T>,
}

impl<T: DataSource> CmdCount<T> {
    pub fn new(store: Rc<T>) -> Self {
        CmdCount { store }
    }
}

impl<T: DataSource> Cmd for CmdCount<T> {
    fn name(&self) -> &str {
        "count"
    }
    fn usage(&self) -> &str {
        "count <all | level>"
    }
    fn description(&self) -> &str {
        "count the grades of one level(or all)"
    }

    fn do_cmd(&self, args: &[String]) -> Result<()> {
        if args.len() != 1 {
            return Err(Error::DoCmdError(String::from("args unmatched.")));
        }
        let stats = if let Ok(level) = args[0].as_str().parse::<u8>() {
            if level < 1 || level > 20 {
                return Err(Error::DoCmdError(String::from("args unmatched.")));
            }
            self.store.get_level_stat(Some(level))
        } else if args[0].as_str() == "all" {
            self.store.get_level_stat(None)
        } else {
            return Err(Error::DoCmdError(String::from("args unmatched.")));
        };
        let mut tab = table!(["level", "S", "AAA+", "AAA", "PUC", "UC", "HC", "NC", "played"]);
        for s in stats.iter() {
            tab.add_row(row![
                s.level(),
                s.s_num(),
                s.tap_num(),
                s.ta_num(),
                s.puc_num(),
                s.uc_num(),
                s.hc_num(),
                s.nc_num(),
                format!("{}/{}", s.played(), self.store.get_level_count(*s.level())),
            ]);
        }
        tab.printstd();
        Ok(())
    }
}
