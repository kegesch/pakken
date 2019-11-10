#[macro_use]
extern crate serde;

pub mod error;
pub mod project;
pub mod target;

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub root: Option<String>,
    pub name: String,
}

pub const PAKKEN_FILE_ENDING: &str = ".pkn";
pub const GENERATOR_FILE_ENDING: &str = ".pgen";
