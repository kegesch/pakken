#[macro_use]
extern crate serde;

use crate::code::GeneratedCode;
use std::io;
use std::path::{Path, PathBuf};

pub mod buffer;
pub mod code;
pub mod error;
pub mod filestructure;
pub mod log;
pub mod project;
pub mod target;

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub path: PathBuf,
}

impl Model {
    pub fn new(path: PathBuf) -> Model { Model { path } }
}

/// Merges `other` into itself.
pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

pub trait Save {
    fn save_at<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error>;
}

pub trait Generate {
    fn generate(&self) -> GeneratedCode;
}

pub const PAKKEN_FILE_ENDING: &str = ".pkn";
pub const GENERATOR_FILE_ENDING: &str = ".pgen";
