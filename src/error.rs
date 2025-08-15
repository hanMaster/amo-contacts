use crate::{amo, profit};
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Config
    ConfigMissingEnv(&'static str),
    ConfigWrongFormat(&'static str),

    AmoCRM(amo::Error),
    // -- Xlsx
    Xlsx(rust_xlsxwriter::XlsxError),
}

// region:    ---From

impl From<rust_xlsxwriter::XlsxError> for Error {
    fn from(value: rust_xlsxwriter::XlsxError) -> Self {
        Error::Xlsx(value)
    }
}

impl From<amo::Error> for Error {
    fn from(e: amo::Error) -> Error {
        Error::AmoCRM(e)
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
