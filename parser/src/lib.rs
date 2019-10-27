extern crate pest;
#[macro_use]
extern crate pest_derive;

use ast::*;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "pakken.pest"]
struct PakkenParser;

pub fn parse_pakken_file(code: &str) -> Result<Vec<Namespace>, Error<Rule>> {
    let pairs = PakkenParser::parse(Rule::pakken, code)?;
    println!("{:?}", pairs);
    let mut namespaces = vec![];
    for pair in pairs {
        if let Rule::namespace = pair.as_rule() {
            namespaces.push(parse_namespace(pair)?)
        }
    }

    Ok(namespaces)
}

fn parse_namespace(namespace: Pair<Rule>) -> Result<Namespace, Error<Rule>> {
    let mut inner_pairs = namespace.into_inner();
    let mut entities = vec![];
    let mut identifier = "default";

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::spacename => identifier = pair.as_str(),
            Rule::entity => entities.push(parse_entity(pair)?),
            _ => return Err(Error::new_from_span(ErrorVariant::CustomError {message: String::from("Not a valid namespace.")},pair.as_span()))
        }
    }
    let namespace = ast::Namespace { identifier: String::from(identifier), entities };

    Ok(namespace)
}

fn parse_entity(entity: Pair<Rule>) -> Result<Entity, Error<Rule>> {
    let attributes = vec![];
    let operations = vec![];

    Ok(Entity { name: String::from("test"), attributes, operations, parent: None })
}

#[cfg(test)]
mod tests {
    use crate::parse_pakken_file;
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
            match parse_pakken_file(code.as_str()) {
                Ok(res) => {
                    assert_eq!(res.len(), 1);
                    let namespace = res.get(0).unwrap();
                    assert_eq!(namespace.identifier, "org.mobile");
                },
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    panic!();
                }
            }
        } else {
            panic!();
        }
    }
}
