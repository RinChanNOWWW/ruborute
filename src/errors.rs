use failure::Fail;
use std::{io, result, string};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "i/o error: {}", _0)]
    IoError(io::Error),
    #[fail(display = "from utf8 error: {}", _0)]
    FromUtf8Error(string::FromUtf8Error),
    #[fail(display = "do command failed: {}", _0)]
    DoCmdError(String),
    #[fail(display = "{}", _0)]
    OtherError(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
