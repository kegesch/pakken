use crate::pesten::Rule;
use displaydoc::Display;
use pest::error::Error as PestError;
use pest::error::ErrorVariant::{CustomError, ParsingError};
use pest::error::LineColLocation::{Pos, Span};
use std::fmt::{Display, Error, Formatter};
use thiserror::Error;

#[derive(Display, Error, Debug)]
pub enum ParserError {
    /// Invalid Token found: `{0}`
    InvalidRule(Rule),

    /// No Token found
    NoToken,

    /// Error at line `{0}`
    Line(usize),

    /// Parsing error at `{0}:{1}`;
    ParsingError(usize, usize),

    /// Parsing error at `{0}:{1}`: `{2}`;
    CustomError(usize, usize, String),

    /// Unhandled Error
    Unhandled,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> { write!(f, "{:?}", self) }
}

impl From<PestError<Rule>> for ParserError {
    fn from(err: PestError<Rule>) -> Self {
        let line_col = match err.line_col {
            Pos((line, col)) => (line, col),
            Span((line0, col0), (_, _)) => (line0, col0),
        };

        match err.variant {
            ParsingError { .. } => ParserError::ParsingError(line_col.0, line_col.1),
            CustomError { message } => ParserError::CustomError(line_col.0, line_col.1, message),
        }
    }
}
