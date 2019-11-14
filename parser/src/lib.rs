extern crate pest;
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

pub mod dangling;
pub mod error;
pub mod parser;
pub mod pesten;

use crate::error::ParserError;
use crate::pesten::Parsable;
use ast::*;
use std::fs;
use std::path::Path;

type ParserResult<T> = std::result::Result<T, ParserError>;

pub fn parse(code: &str) -> ParserResult<Namespace> {
    Namespace::pest_parse(pesten::Rule::namespace, code)
}

pub fn parse_from_file<P: AsRef<Path>>(file: P) -> ParserResult<Namespace> {
    if let Ok(code) = fs::read_to_string(file) {
        parse(code.as_str())
    } else {
        Err(ParserError::FileNotFound)
    }
}

#[cfg(tests)]
mod tests {
    use crate::parse;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parser() {
        let path = Path::new("./test/example.pakken");
        println!("{}", path.display());
        let file = fs::read_to_string(path.canonicalize().unwrap());
        println!("{}", path.canonicalize().unwrap().display());
        println!("{:?}", file);
        if let Ok(code) = file {
            match parse(code.as_str()) {
                Ok(res) => {
                    assert_eq!(res.len(), 1);
                    let namespace = res.get(0).unwrap();
                    assert_eq!(namespace.identifier, "org.mobile");
                },
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    panic!();
                },
            }
        } else {
            panic!();
        }
    }
}
