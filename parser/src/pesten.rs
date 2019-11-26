use crate::error::ParserError;
use crate::ParserResult;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

pub type PakkenRule = Rule;

#[derive(Parser)]
#[grammar = "pakken.pest"]
struct PakkenParser;

pub trait Parsable: Sized {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self>;

    fn pest_parse(rule: Rule, code: &str) -> ParserResult<Self> {
        let result: Option<Pair<Rule>> = PakkenParser::parse(rule, code)?.next();
        Self::from_pest(result.ok_or(ParserError::NoToken)?)
    }
}

pub fn lex(code: &str) -> ParserResult<()> {
    PakkenParser::parse(Rule::pakken, code)?;
    Ok(())
}
