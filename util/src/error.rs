use displaydoc::Display;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum PakError {
    /// error: {0}
    CustomError(String),
    /// io-error: {0}
    IoError(io::Error),
    /// this is not a pakken project.
    NotAProject,
    /// could not read project file
    ProjectReadError,
    /// serialization error: {0}
    SerializationError(ron::ser::Error),
    /// could not locate the target `{0}`
    TargetNotFound(String),
    /// parser error: {0}
    ParserError(String),
}

impl From<io::Error> for PakError {
    fn from(err: io::Error) -> Self { PakError::IoError(err) }
}

impl From<ron::ser::Error> for PakError {
    fn from(err: ron::ser::Error) -> Self { PakError::SerializationError(err) }
}

pub type PakResult<T> = Result<T, PakError>;
