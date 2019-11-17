#[macro_use]
extern crate serde;
use crate::buffer::Buffer;
use difference::Changeset;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub mod buffer;
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

#[derive(Debug, Clone)]
pub enum FileStructure {
    /// Name, Content
    File(String, CodePage),
    /// Name, Content
    Dir(String, Vec<FileStructure>),
}

/// Merges `other` into itself.
pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

pub trait Save {
    fn save_at<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error>;
}

impl Save for FileStructure {
    fn save_at<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let p = path.as_ref();
        match self {
            FileStructure::Dir(name, content) => {
                let path = Path::new(p).join(name);
                let path_ref = path.as_path();

                if let Err(err) = fs::create_dir(path_ref) {
                    if err.kind() != ErrorKind::AlreadyExists {
                        return Err(err);
                    }
                }
                for fs in content {
                    fs.save_at(path_ref)?;
                }
                Ok(())
            },
            FileStructure::File(name, content) => {
                let path = Path::new(p).join(name);
                let path_ref = path.as_path();
                let mut file_handle =
                    OpenOptions::new().write(true).truncate(true).create(true).open(path_ref)?;
                file_handle.write_all(content.build().as_bytes())?;
                Ok(())
            },
        }
    }
}

impl Merge for FileStructure {
    fn merge(&self, other: &Self) -> Self {
        match self {
            FileStructure::File(name, content) => {
                if let FileStructure::File(other_name, other_content) = other {
                    if name == other_name {
                        let new_content = content.merge(other_content);
                        return FileStructure::File(name.clone(), new_content);
                    }
                }
                FileStructure::Dir("./".to_owned(), vec![self.clone(), other.clone()])
            },
            FileStructure::Dir(name, content) => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodePage {
    pub fragments: Vec<CodeFragment>,
    pub comment_string: &'static str,
}

impl CodePage {
    fn default(comment_string: &'static str) -> CodePage {
        CodePage { fragments: vec![], comment_string }
    }

    fn add(&mut self, fragment: CodeFragment) { self.fragments.push(fragment); }

    fn build(&self) -> String {
        let mut buffer = Buffer::default();
        for fragment in self.fragments {
            buffer.new_line();
            match fragment {
                CodeFragment::Generated(generated) => {
                    buffer += self.comment_string;
                    buffer += " @GENERATED";
                    buffer.new_line();
                    buffer += generated.code.as_str();
                    buffer.new_line();
                    buffer += self.comment_string;
                    buffer += " @END";
                },
                CodeFragment::Other(code) => {
                    buffer += code.as_str();
                },
            }
            buffer.new_line();
        }
        buffer.flush()
    }

    fn get_generated(&self, code_id: String) -> Option<GeneratedCode> {
        for frag in self.fragments {
            if let CodeFragment::Generated(generated) = frag {
                if generated.id == code_id {
                    return Some(generated);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum CodeFragment {
    Generated(GeneratedCode),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub code: String,
    pub id: String,
}

impl GeneratedCode {
    fn to_fragment(&self) -> CodeFragment { CodeFragment::Generated(self.clone()) }
}

pub trait Fragment {
    fn fragment(&self) -> CodeFragment;
}

impl Merge for CodePage {
    fn merge(&self, other: &Self) -> Self {
        let mut merged_fragments = vec![];
        let mut new_code_page = CodePage::default(self.comment_string);

        for frag in self.fragments {
            match frag {
                CodeFragment::Generated(code) => {
                    if let Some(other_generated) = other.get_generated(code.id) {
                        new_code_page.add(CodeFragment::Generated(other_generated));
                    }
                },
                _ => {
                    new_code_page.add(frag);
                },
            }
        }

        new_code_page
    }
}

pub trait Generate {
    fn generate(&self) -> GeneratedCode;
}

pub const PAKKEN_FILE_ENDING: &str = ".pkn";
pub const GENERATOR_FILE_ENDING: &str = ".pgen";
