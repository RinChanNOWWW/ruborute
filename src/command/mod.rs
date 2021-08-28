use crate::Result;

mod command;

/// the Cmd trait is used to add into Cmdline.
///
/// `name`, `usage`, `description` is used by `Cmdline.help()`
pub trait Cmd {
    /// name of the command
    fn name(&self) -> &str;
    /// usage of the command
    fn usage(&self) -> &str;
    /// descripton of the command
    fn description(&self) -> &str;
    /// do the command
    fn do_cmd(&self, args: &[String]) -> Result<()>;
}

pub use self::command::*;
