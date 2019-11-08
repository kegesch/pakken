use displaydoc::Display;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum PakError {
    /// Error: `{0}`
    CustomError(String),
    /// IOError: `{0}`
    IoError(io::Error),
}

impl From<io::Error> for PakError {
    fn from(err: io::Error) -> Self { PakError::IoError(err) }
}

pub type PakResult<T> = Result<T, PakError>;
