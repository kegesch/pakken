use crate::error::{PakError, PakResult};
use crate::Model;
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub model: Model,
}

const PROJECT_FILE_NAME: &str = ".pakken.ron";

impl Project {
    pub fn from(name: &str) -> Project {
        let path = Path::new("./").join(name).canonicalize().unwrap();
        //let mut default_model_file = String::from(name);
        //default_model_file.push_str(PAKKEN_FILE_ENDING);
        //let model_path = PathBuf::from(default_model_file.as_str());
        let model = Model { name: String::from(name), root: None };
        Project { name: String::from(name), path, model }
    }

    pub fn save(&self) -> PakResult<()> {
        let se = to_string_pretty(self, PrettyConfig::default())?;
        let content = se.as_bytes();
        let path = self.path.join(PROJECT_FILE_NAME);
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        let res = file.write_all(content);
        if let Err(_) = res {
            Err(PakError::CustomError(String::from("Could not save project file.")))
        } else {
            Ok(())
        }
    }

    pub fn read() -> PakResult<Project> {
        let path = Path::new("./").join(PROJECT_FILE_NAME);
        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let res = from_str(content.as_str());
                if let Ok(project) = res {
                    Ok(project)
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
}
