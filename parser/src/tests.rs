use crate::parse;
use crate::pesten::lex;
use std::fs;
use std::path::Path;

#[test]
/// ensures that the grammar is correct
fn test_lexer() {
    let path = Path::new("./test/example.pakken");
    let file = fs::read_to_string(path.canonicalize().unwrap());
    if let Ok(code) = file {
        let res = lex(&code);
        assert!(res.is_ok(), true);
    } else {
        panic!();
    }
}

#[test]
/// ensures that the ast is parsed correctly
fn test_parser() {
    let path = Path::new("./test/example.pakken");
    let file = fs::read_to_string(path.canonicalize().unwrap());
    if let Ok(code) = file {
        match parse(code.as_str()) {
            Ok(namespace) => {
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
