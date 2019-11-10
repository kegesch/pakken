use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use util::error::{PakError, PakResult};
use util::project::Project;
use util::target::TargetRepository;
use util::GENERATOR_FILE_ENDING;

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    target_name: String,
    path: PathBuf,
}
// TODO path structure?

impl Generator {
    pub fn generate(&self, target_repo: &TargetRepository) -> PakResult<()> {
        // generate new code
        let project = Project::read()?;
        let model = project.model;
        let target = target_repo.find(self.target_name.as_str())?;
        target.generate_from(model)?;

        // merge

        // save
        Ok(())
    }

    pub fn new<P: AsRef<Path>>(target: &str, out_dir: P) -> Generator {
        Generator { path: PathBuf::from(out_dir.as_ref()), target_name: String::from(target) }
    }

    pub fn from(path: &Path) -> PakResult<Generator> {
        if path.exists() {
            if let Ok(mut file) = File::open(path) {
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                let res = from_str(content.as_str());
                if let Ok(generator) = res {
                    Ok(generator)
                } else {
                    Err(PakError::CustomError(String::from("Could not read generator file.")))
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
        let project = Project::read()?;
        let model = project.model;
        let mut name_file = model.name.clone();
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

pub trait Transform<M> {
    fn transform(model: &M) -> Self;
}

pub trait Printer {
    fn serialize(&self) -> PakResult<String>;
}
