use crate::error::{PakError, PakResult};
use crate::{Model, GENERATOR_FILE_ENDING};
use ast::Namespace;
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub enum TargetLocation {
    GitHub(String),
    Local(PathBuf),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    name: String,
    location: TargetLocation,
}

impl Target {
    pub(crate) fn from(name: &str) -> Target {
        let location = TargetLocation::Local(PathBuf::from("./targets").join(name));
        Target { name: String::from(name), location }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Generator {
    model: Model,
    target: Target,
}

impl Generator {
    pub(crate) fn generate(&self) -> PakResult<()> {
        // TODO scaffold and generate code merge / diff
        println!("ASKLJDÖALKSDJ");
        Ok(())
    }

    pub fn new(model: Model, target: Target) -> Generator { Generator { model, target } }

    pub fn from(path: &Path) -> PakResult<Generator> {
        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let res = from_str(content.as_str());
                if let Ok(generator) = res {
                    Ok(generator)
                } else {
                    Err(PakError::CustomError(String::from("Could not read Project file.")))
                }
            } else {
                Err(PakError::ProjectReadError)
            }
        } else {
            Err(PakError::NotAProject)
        }
    }

    pub fn save(&self) -> PakResult<()> {
        let se = to_string_pretty(self, PrettyConfig::default())?;
        let content = se.as_bytes();
        let mut name_file = self.model.name.clone();
        name_file.push_str(GENERATOR_FILE_ENDING);
        let path = Path::new("./").join(name_file);
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        let res = file.write_all(content);
        if let Err(_) = res {
            Err(PakError::CustomError(String::from("Could not save project file.")))
        } else {
            Ok(())
        }
    }
}

pub trait Generative<M> {
    fn generate(model: M) -> Self;
}
