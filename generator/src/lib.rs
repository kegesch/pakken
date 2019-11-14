use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};
use util::error::{PakError, PakResult};
use util::project::Project;
use util::target::TargetRepository;
use util::{Model, Save, GENERATOR_FILE_ENDING};

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
        target.generate_from(model)?.save_at(self.path.as_path())?;

        // merge

        // save
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

pub struct Buffer {
    buffer: String,
    indents: i8,
    indent_string: &'static str,
}

impl Buffer {
    pub fn default() -> Buffer { Buffer { buffer: String::new(), indent_string: "\t", indents: 0 } }

    pub fn indent(&mut self) { self.indents += 1; }

    pub fn unindent(&mut self) { self.indents -= 1; }

    pub fn new_line(&mut self) {
        self.buffer.push_str("\n");
        for _ in 0 .. self.indents {
            self.buffer.push_str(self.indent_string);
        }
    }

    pub fn flush(self) -> String { self.buffer }
}

impl Add<&'_ str> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: &'_ str) -> Self::Output {
        self.buffer.push_str(rhs);
        self
    }
}

impl Add<Buffer> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: Buffer) -> Self::Output {
        self.buffer.push_str(rhs.flush().as_str());
        self
    }
}

impl AddAssign<&str> for Buffer {
    fn add_assign(&mut self, rhs: &str) { self.buffer.push_str(rhs); }
}

pub trait Printer {
    fn serialize(&self, buffer: Buffer) -> Buffer;
}
