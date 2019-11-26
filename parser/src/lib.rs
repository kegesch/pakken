extern crate pest;
extern crate pest_derive;

pub mod dangling;
pub mod error;
pub mod parser;
pub mod pesten;

#[cfg(test)]
mod tests;

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
