use crate::error::ParserError;
use crate::pesten::{Parsable, Rule};
use crate::ParserResult;
use ast::{Entity, Namespace};
use pest::iterators::Pair;

impl Parsable for Namespace {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let mut entities = vec![];
        let identifier = String::from(
            inner_pairs.next().expect("Namespace should always have an identifier.").as_str(),
        );

        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::entity => {
                    println!("parse entity entity");
                },
                other => return Err(ParserError::InvalidRule(other)),
            }
        }
        let namespace = ast::Namespace { identifier, entities };

        Ok(namespace)
    }
}
