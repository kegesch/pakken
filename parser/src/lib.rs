extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use pest::iterators::Pair;
use ast::*;

#[derive(Parser)]
#[grammar = "pakken.pest"]
struct PakkenParser;

pub fn parse_pakken_file(file: &str) -> Result<Vec<Namespace>, Error<Rule>> {
    let pairs = PakkenParser::parse(Rule::pakken, file)?;
    let mut namespaces = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::pakken => namespaces.push(parse_pakken(pair)?),
            _ => {}
        }
    }

    Ok(namespaces)
}

fn parse_pakken(namespace: Pair<Rule>) -> Result<Namespace, Error<Rule>> {
    let mut inner_pairs = namespace.into_inner();
    let mut entities = vec![];
    let identifier = inner_pairs.next().unwrap().as_str();
    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::entity => entities.push(parse_entity(pair)?),
            _ => {}
        }
    }
    let namespace = ast::Namespace {identifier: String::from(identifier), entities};

    Ok(namespace)
}

fn parse_entity(entity: Pair<Rule>) -> Result<Entity, Error<Rule>> {
    let attributes = vec![];
    let operations = vec![];

    Ok(Entity { name: String::from("test"), attributes, operations, parent: None})
}