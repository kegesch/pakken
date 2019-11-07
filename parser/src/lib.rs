extern crate pest;
extern crate pest_derive;

pub mod error;
pub mod linker;
pub mod parser;
pub mod pesten;

use crate::error::ParserError;
use crate::pesten::Parsable;
use ast::*;

type ParserResult<T> = std::result::Result<T, ParserError>;

pub fn parse(code: &str) -> ParserResult<Namespace> {
    Namespace::pest_parse(pesten::Rule::namespace, code)
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
