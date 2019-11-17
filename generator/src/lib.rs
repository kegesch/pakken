use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};
use util::buffer::Buffer;
use util::error::{PakError, PakResult};
use util::project::Project;
use util::target::TargetRepository;
use util::{Merge, Model, Save, GENERATOR_FILE_ENDING};

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {
    target_name: String,
    path: PathBuf,
    options: Option<TargetOptions>,
}

pub struct GeneratorBuilder {
    target_name: String,
    options: Option<TargetOptions>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct TargetOptions; // This will contain all the options for a target

impl Generator {
    pub fn generate(&self, target_repo: &TargetRepository) -> PakResult<()> {
        // generate new code
        let project = Project::read()?;
        let model = Model::new(project.model);
        let target = target_repo.find(self.target_name.as_str())?;
        let generated = target.generate_from(model)?;
        // diffing
        let merged = target.load().merge(generated);
        // save
        merged.save_at(self.path.as_path())?;
        Ok(())
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
        let mut name_file = self.target_name.clone();
        name_file.push_str(GENERATOR_FILE_ENDING);
        let path = Path::new("./").join(name_file);
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        let res = file.write_all(content);
        if res.is_err() {
            Err(PakError::CustomError(String::from("Could not save project file.")))
        } else {
            Ok(())
        }
    }
}

impl GeneratorBuilder {
    pub fn new(target: &str) -> GeneratorBuilder {
        GeneratorBuilder { target_name: String::from(target), options: None }
    }

    pub fn with_options(&mut self, options: TargetOptions) -> &mut Self {
        self.options = Some(options);
        self
    }

    pub fn build<P: AsRef<Path>>(&self, out_dir: P) -> Generator {
        let path = PathBuf::from(out_dir.as_ref());
        Generator { path, target_name: self.target_name.clone(), options: self.options }
    }
}

pub trait Transform<M> {
    fn transform(model: &M) -> Self;
}
