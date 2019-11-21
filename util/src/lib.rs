#[macro_use]
extern crate serde;
use crate::buffer::Buffer;
use crate::error::PakResult;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
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

impl FileStructure {
    pub fn find<'a>(&self, other: &'a FileStructure) -> Option<&'a FileStructure> {
        if let FileStructure::Dir(name, content) = self {
            for fs in content {
                match fs {
                    FileStructure::File(name, _content) => {
                        if let FileStructure::File(other_name, _other_content) = other {
                            if name == other_name {
                                return Some(other);
                            }
                        }
                    },
                    FileStructure::Dir(name, _content) => {
                        if let FileStructure::Dir(other_name, _other_content) = other {
                            if name == other_name {
                                return Some(other);
                            }
                        }
                    },
                }
            }
        }
        None
    }

    pub fn load_shadow_from<P: AsRef<Path>>(&self, path: P) -> PakResult<Option<FileStructure>> {
        match self {
            FileStructure::Dir(name, content) => {
                let shadowed = path.as_ref().join(name);
                if shadowed.exists() {
                    let mut shadowed_content = vec![];
                    for fs in content {
                        if let Some(new_fs) = fs.load_shadow_from(shadowed.as_path())? {
                            shadowed_content.push(new_fs);
                        }
                    }

                    Ok(Some(FileStructure::Dir(name.clone(), shadowed_content)))
                } else {
                    Ok(None)
                }
            },
            FileStructure::File(name, code) => {
                let shadowed = path.as_ref().join(name);
                if shadowed.exists() {
                    let mut file = File::open(shadowed)?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    Ok(Some(FileStructure::File(
                        name.clone(),
                        CodePage::from(code.comment_string, buffer.as_str()),
                    )))
                } else {
                    Ok(None)
                }
            },
        }
    }
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
            FileStructure::Dir(name, content) => {
                if let FileStructure::Dir(other_name, other_content) = other {
                    if name == other_name {
                        let mut merged_content = vec![];
                        for fs in content {
                            if let Some(found) = other.find(fs) {
                                merged_content.push(fs.merge(found));
                            } else {
                                merged_content.push(fs.clone());
                            }
                        }
                        return FileStructure::Dir(name.clone(), merged_content);
                    }
                    return FileStructure::Dir("./".to_string(), vec![self.clone(), other.clone()]);
                }

                FileStructure::Dir("./".to_string(), vec![self.clone(), other.clone()])
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodePage {
    pub fragments: Vec<CodeFragment>,
    pub comment_string: &'static str,
}

impl CodePage {
    const DESCRIPTION: &'static str = " THIS FILE IS GENERATED BY PAKKEN. EVERY FRAGMENT MARKED \
                                       WITH @GENERATED WILL BE OVERRIDEN ON THE NEXT GENERATION. \
                                       TO PREVENT THIS CHANGE IT TO @NOT.";
    const END_GENERATED: &'static str = "@END";
    const START_GENERATED: &'static str = "@GENERATED";
    const START_NOT_GENERATED: &'static str = "@NOT GENERATED";

    pub fn default(comment_string: &'static str) -> CodePage {
        CodePage { fragments: vec![], comment_string }
    }

    pub fn from(comment_string: &'static str, code: &str) -> CodePage {
        let mut codepage = CodePage::default(comment_string);
        let gen_start = Self::start_generated_str(comment_string);
        let gen_start_str = gen_start.as_str();
        let gen_end = Self::end_comment_str(comment_string);
        let gen_end_str = gen_end.as_str();
        let mut next = code.find(gen_start_str);
        let mut next_code = code;

        while next.is_some() {
            next_code = next_code.split_at(next.expect("Should have next.")).1;
            let split = next_code.split_at(next_code.find('\n').expect("Should have a new line."));
            let new_code = split.1;
            let comment = split.0;
            let end_code = new_code.find(gen_end_str).expect("Should have an end!");
            let fragment_code = new_code.split_at(end_code).0.trim();
            let fragment_id = comment.split_at(gen_start_str.len()).1.trim();
            let fragment = CodeFragment::Generated(GeneratedCode {
                id: fragment_id.to_string(),
                code: fragment_code.to_string(),
            });
            codepage.add(fragment);

            let not_generated = new_code.split_at(end_code).1;
            let start =
                not_generated.split_at(not_generated.find('\n').expect("Should be next line")).1;
            if let Some(end_index) = start.find(gen_start_str) {
                let custom_code = start.split_at(end_index).0.trim();
                if !custom_code.is_empty() {
                    codepage.add(CodeFragment::Other(custom_code.to_string()))
                }
            }
            // TODO handle not generated code fragment and check with id
            next = start.find(gen_start_str);

            next_code = start;
        }

        codepage
    }

    pub fn add(&mut self, fragment: CodeFragment) { self.fragments.push(fragment); }

    fn start_generated_str(comment_string: &str) -> String {
        let mut buffer = Buffer::default();
        buffer += comment_string;
        buffer += " ";
        buffer += Self::START_GENERATED;
        buffer.flush()
    }

    fn start_not_generated_str(comment_string: &str) -> String {
        let mut buffer = Buffer::default();
        buffer += comment_string;
        buffer += " ";
        buffer += Self::START_NOT_GENERATED;
        buffer.flush()
    }

    fn end_comment_str(comment_string: &str) -> String {
        let mut buffer = Buffer::default();
        buffer += comment_string;
        buffer += " ";
        buffer += Self::END_GENERATED;
        buffer.flush()
    }

    fn build(&self) -> String {
        let mut buffer = Buffer::default();
        buffer += self.comment_string;
        buffer += Self::DESCRIPTION;
        for fragment in &self.fragments {
            buffer.new_line();
            match fragment {
                CodeFragment::Generated(generated) => {
                    buffer += Self::start_generated_str(self.comment_string).as_str();
                    buffer += " ";
                    buffer += generated.id.as_str();
                    buffer.new_line();
                    buffer += generated.code.as_str();
                    buffer.new_line();
                    buffer += Self::end_comment_str(self.comment_string).as_str();
                },
                CodeFragment::Other(code) => {
                    buffer += code.as_str();
                },
                CodeFragment::NotGenerated(not_generated) => {
                    buffer += Self::start_not_generated_str(self.comment_string).as_str();
                    buffer += not_generated.id.as_str();
                    buffer.new_line();
                    buffer += not_generated.code.as_str();
                    buffer.new_line();
                    buffer += Self::end_comment_str(self.comment_string).as_str();
                },
            }
            buffer.new_line();
        }
        buffer.flush()
    }

    fn get_generated(&self, code_id: &str) -> Option<GeneratedCode> {
        for frag in &self.fragments {
            if let CodeFragment::Generated(generated) = frag {
                if generated.id == code_id.to_string() {
                    return Some(generated.clone());
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum CodeFragment {
    Generated(GeneratedCode),
    NotGenerated(GeneratedCode),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub code: String,
    pub id: String,
}

impl GeneratedCode {
    pub fn to_fragment(&self) -> CodeFragment { CodeFragment::Generated(self.clone()) }
}

pub trait Fragment {
    fn fragment(&self) -> CodeFragment;
}

impl Merge for CodePage {
    fn merge(&self, other: &Self) -> Self {
        let mut new_code_page = CodePage::default(self.comment_string);

        for frag in &self.fragments {
            match frag {
                CodeFragment::Generated(code) => {
                    if let Some(other_generated) = other.get_generated(&code.id) {
                        new_code_page.add(CodeFragment::Generated(other_generated));
                    }
                },
                _ => {
                    new_code_page.add(frag.clone());
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
