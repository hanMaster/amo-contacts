use crate::profit;
use std::fmt::{Display, Formatter};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Funnels(String),
    GetContactFailed(String),
    Profitbase(profit::Error),
}

// region:    ---From
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Request(e)
    }
}

impl From<profit::Error> for Error {
    fn from(e: profit::Error) -> Self {
        Error::Profitbase(e)
    }
}
// endregion: ---From

// region:    --- Error boilerplate
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
// endregion: --- Error boilerplate
