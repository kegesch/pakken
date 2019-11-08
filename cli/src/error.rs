use displaydoc::Display;
use std::io;
use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum PakError {
    /// Error: `{0}`
    CustomError(String),
    /// IOError: `{0}`
    IoError(io::Error),
    /// This is not a pakken project.
    NotAProject,
    /// Could not read project file
    ProjectReadError,
    /// SerializationError: `{0}`
    SerializationError(ron::ser::Error),
}

impl From<io::Error> for PakError {
    fn from(err: io::Error) -> Self { PakError::IoError(err) }
}

impl From<ron::ser::Error> for PakError {
    fn from(err: ron::ser::Error) -> Self { PakError::SerializationError(err) }
}

pub type PakResult<T> = Result<T, PakError>;
