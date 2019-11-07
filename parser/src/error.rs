use crate::pesten::Rule;
use displaydoc::Display;
use std::fmt::{Display, Error, Formatter};
use thiserror::Error;

#[derive(Display, Error, Debug)]
pub enum ParserError {
    /// Invalid Token found: `{0}`
    InvalidRule(Rule),

    /// No Token found
    NoToken,

    /// Error at line `{0}`
    Line(u8),

    /// Unhandled Error
    Unhandled,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { Ok(print!("{:?}", self)) }
}
