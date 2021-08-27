use crate::Result;

mod command;

pub trait Cmd {
    fn name(&self) -> &str;
    fn usage(&self) -> &str;
    fn description(&self) -> &str;
    fn do_cmd(&self, args: &[String]) -> Result<()>;
}

pub use self::command::*;
