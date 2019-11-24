use crate::buffer::Buffer;
use crate::Merge;

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
    const START_NOT_GENERATED: &'static str = "@NOT";

    pub fn default(comment_string: &'static str) -> CodePage {
        CodePage { fragments: vec![], comment_string }
    }

    pub fn from(comment_string: &'static str, code: &str) -> CodePage {
        let mut codepage = CodePage::default(comment_string);
        let removed_comment =
            code.split_at(code.find('\n').expect("There should be at least two lines.")).1.trim();
        let splitted_ends = Self::split_ends(comment_string, removed_comment);
        let fragments = Self::split_fragments(comment_string, splitted_ends);
        for frag in fragments {
            codepage.add(frag);
        }
        codepage
    }

    fn split_ends<'a>(comment_string: &'static str, code: &'a str) -> Vec<&'a str> {
        let splitted: Vec<&str> =
            code.split(Self::end_comment_str(comment_string).as_str()).collect();
        splitted.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).collect()
    }

    fn split_fragments(
        comment_string: &'static str, splitted_ends: Vec<&str>,
    ) -> Vec<CodeFragment> {
        let mut code_fragments = vec![];

        let gen_start = Self::start_generated_str(comment_string);
        let gen_start_str = gen_start.as_str();
        let not_gen_start = Self::start_not_generated_str(comment_string);
        let not_gen_start_str = not_gen_start.as_str();

        for split in splitted_ends {
            // Each split can either contain only code for one (Not)GeneratedCode or additional code for CodeFragment::Other()

            let split_location;
            let is_generated;
            if let Some(found) = split.find(gen_start_str) {
                split_location = found;
                is_generated = true;
            } else if let Some(found) = split.find(not_gen_start_str) {
                split_location = found;
                is_generated = false;
            } else {
                // This should not happen!!
                panic!("Found an end without heading @generated or @not");
            }

            let new_split = split.split_at(split_location);
            let before = new_split.0.trim();
            let after = new_split.1;
            let comment_code = after.split_at(after.find('\n').expect("Should have next line."));
            let code = comment_code.1.trim();
            if is_generated {
                let id = comment_code.0.split_at(gen_start_str.len()).1.trim();
                let fragment = CodeFragment::Generated(GeneratedCode {
                    id: id.to_string(),
                    code: code.to_string(),
                });
                code_fragments.push(fragment);
            } else {
                let id = comment_code.0.split_at(not_gen_start_str.len()).1.trim();
                let fragment = CodeFragment::NotGenerated(GeneratedCode {
                    id: id.to_string(),
                    code: code.to_string(),
                });
                code_fragments.push(fragment);
            }

            if !before.is_empty() {
                code_fragments.push(CodeFragment::Other(before.to_string()));
            }
        }
        code_fragments
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

    pub(crate) fn build(&self) -> String {
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
                    buffer += " ";
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

    fn get_not_generated(&self, code_id: &str) -> Option<GeneratedCode> {
        for frag in &self.fragments {
            if let CodeFragment::NotGenerated(generated) = frag {
                if generated.id == code_id {
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
                CodeFragment::NotGenerated(_code) => {
                    new_code_page.add(frag.clone());
                },
                CodeFragment::Other(_code) => {
                    new_code_page.add(frag.clone());
                },
                _ => (),
            }
        }
        for frag in &other.fragments {
            if let CodeFragment::Generated(code) = frag {
                let found_not_generated = new_code_page.get_not_generated(code.id.as_str());
                if found_not_generated.is_none() {
                    new_code_page.add(frag.clone());
                }
            }
        }

        new_code_page
    }
}
