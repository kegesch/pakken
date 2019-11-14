#[macro_use]
extern crate serde;

use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub mod error;
pub mod project;
pub mod target;

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub path: PathBuf,
}

impl Model {
    pub fn new(path: PathBuf) -> Model { Model { path } }
}

pub enum FileStructure {
    /// Name, Content
    File(String, String),
    /// Name, Content
    Dir(String, Vec<FileStructure>),
}

pub trait Save {
    fn save_at<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error>;
}

impl Save for FileStructure {
    fn save_at<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let p = path.as_ref();
        println!("saving at {}", p.display());
        match self {
            FileStructure::Dir(name, content) => {
                let path = Path::new(p).join(name);
                let path_ref = path.as_path();

                println!("trying to create dir at {}", path_ref.display());
                if let Err(err) = fs::create_dir(path_ref) {
                    if err.kind() != ErrorKind::AlreadyExists {
                        return Err(err);
                    }
                }
                println!("created dir at {}", path_ref.display());
                for fs in content {
                    fs.save_at(path_ref)?;
                }
                Ok(())
            },
            FileStructure::File(name, content) => {
                let path = Path::new(p).join(name);
                let path_ref = path.as_path();
                println!("trying to create file at {}", path_ref.display());
                let mut file_handle =
                    OpenOptions::new().write(true).truncate(true).create(true).open(path_ref)?;
                println!("trying to dump file at {}", path_ref.display());
                file_handle.write_all(content.as_bytes())?;
                println!("dunped at {}", path_ref.display());
                Ok(())
            },
        }
    }
}

pub const PAKKEN_FILE_ENDING: &str = ".pkn";
pub const GENERATOR_FILE_ENDING: &str = ".pgen";
