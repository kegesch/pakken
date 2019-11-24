use crate::code::CodePage;
use crate::error::PakResult;
use crate::{Merge, Save};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum FileStructure {
    /// Name, Content
    File(String, CodePage),
    /// Name, Content
    Dir(String, Vec<FileStructure>),
}

impl FileStructure {
    pub fn get_name(&self) -> &str {
        match self {
            FileStructure::File(name, _content) => name,
            FileStructure::Dir(name, _content) => name,
        }
    }

    pub fn find<'a, 'b>(&'a self, name: &'b str) -> Option<(usize, &'a FileStructure)> {
        if let FileStructure::Dir(_name, content) = self {
            Self::find_in_content(content, name)
        } else {
            None
        }
    }

    pub fn find_in_content<'a, 'b>(
        content: &'a Vec<FileStructure>, name: &'b str,
    ) -> Option<(usize, &'a FileStructure)> {
        for (index, fs) in content.iter().enumerate() {
            match fs {
                FileStructure::File(fs_name, _content) => {
                    if fs_name == name {
                        return Some((index, fs));
                    }
                },
                FileStructure::Dir(fs_name, _content) => {
                    if fs_name == name {
                        return Some((index, fs));
                    }
                },
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
                        for fs in other_content {
                            if let Some((index, found)) = self.find(fs.get_name()) {
                                merged_content.insert(index, found.merge(fs));
                            } else {
                                merged_content.push(fs.clone());
                            }
                        }
                        for fs in content {
                            if FileStructure::find_in_content(&merged_content, fs.get_name())
                                .is_none()
                            {
                                merged_content.push(fs.clone())
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
