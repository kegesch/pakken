use crate::error::ParserError;
use crate::ParserResult;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "pakken.pest"]
struct PakkenParser;

pub trait Parsable: Sized {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self>;

    fn from_pest_opt(pair_opt: Option<Pair<Rule>>) -> ParserResult<Self> {
        Self::from_pest(pair_opt.ok_or(ParserError::NoToken)?)
    }

    fn pest_parse(rule: Rule, code: &str) -> ParserResult<Self> {
        let result = PakkenParser::parse(rule, code)?.next();
        Self::from_pest_opt(result)
    }
}
