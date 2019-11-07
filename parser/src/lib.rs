extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod error;
pub mod parser;
pub mod pesten;

use crate::error::ParserError;
use crate::pesten::Parsable;
use ast::*;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
use pest::{Parser, RuleType};

type ParserResult<T> = std::result::Result<T, ParserError>;

pub fn parse(code: &str) -> ParserResult<Namespace> {
    Namespace::pest_parse(pesten::Rule::namespace, code)
}

/*
fn parse_namespace(namespace: Pair<Rule>) -> Result<Namespace, Error<Rule>> {
    let inner_pairs = namespace.into_inner();
    let mut entities = vec![];
    let mut identifier = String::from("default");

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::spacename => identifier = String::from(pair.as_str()),
            Rule::entity => {
                let parsed = parse_entity(pair, &entities)?;
                entities.push(parsed);
            },
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError { message: String::from("Not a valid namespace.") },
                    pair.as_span(),
                ))
            },
        }
    }
    let namespace = ast::Namespace { identifier, entities };

    Ok(namespace)
}

fn parse_entity(
    entity: Pair<Rule>, available_entities: &Vec<Entity>,
) -> Result<Entity, Error<Rule>> {
    let inner_pairs = entity.into_inner();
    let mut attributes: Vec<Attribute> = vec![];
    let mut operations: Vec<Operation> = vec![];
    let mut name = String::from("default");
    let mut parent_identifier: Option<Identifier> = None;

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::entityname => name = String::from(pair.as_str()),
            Rule::parententityname => {
                let e_name = String::from(pair.as_str());
                if let Some(parent_entity) =
                    available_entities.into_iter().find(|e| e.name.eq(&e_name))
                {
                    parent_identifier = Some(parent_entity.name.clone());
                }
            },
            Rule::feature => {
                let inner_feature = pair.into_inner();
                for feature_pair in inner_feature {
                    match feature_pair.as_rule() {
                        Rule::attribute => {
                            attributes.push(parse_attribute(feature_pair, available_entities)?)
                        }, // TODO add self to available_entities?
                        Rule::operation => {
                            operations.push(parse_operation(feature_pair, available_entities)?)
                        },
                        _ => {
                            return Err(Error::new_from_span(
                                ErrorVariant::CustomError {
                                    message: String::from("Not a valid feature."),
                                },
                                pair.as_span(),
                            ))
                        },
                    }
                }
            },
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError { message: String::from("Not a valid entity.") },
                    pair.as_span(),
                ))
            },
        }
    }

    let parsed = Entity { name, attributes, operations, parent_identifier };
    Ok(parsed)
}

fn parse_attribute(
    attribute: Pair<Rule>, available_entities: &Vec<Entity>,
) -> Result<Attribute, Error<Rule>> {
    let inner_pairs = attribute.into_inner();

    let mut name = String::from("default");
    let mut entity_identifier: Option<Identifier> = None;
    let mut multiplicity = Multiplicity::Single;

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::attributename => name = String::from(pair.as_str()),
            Rule::entityname => {
                let e_name = String::from(pair.as_str());
                if let Some(entity_type) =
                    available_entities.into_iter().find(|e| e.name.eq(&e_name))
                {
                    entity_identifier = Some(entity_type.name);
                }
            },
            Rule::multiplicity => multiplicity = parse_multiplicity(pair)?,
            _ => {
                return Err(Error::new_from_span(
                    ErrorVariant::CustomError { message: String::from("Not a valid attribute.") },
                    pair.as_span(),
                ))
            },
        }
    }

    if let Some(e_type) = entity_identifier {
        Ok(Attribute { name, entity_identifier: e_type, multiplicity })
    } else {
        Err(Error::new_from_span(
            ErrorVariant::CustomError { message: String::from("Could not find entity.") },
            attribute.as_span(),
        ))
    }
}

fn parse_multiplicity(multiplicity: Pair<Rule>) -> Result<Multiplicity, Error<Rule>> {
    let inner_pairs = multiplicity.into_inner();

    println!("{:?}", inner_pairs);

    Ok(Multiplicity::Single)
}

fn parse_operation(
    attribute: Pair<Rule>, available_entities: &Vec<Entity>,
) -> Result<Operation, Error<Rule>> {
    Ok(Operation {
        name: String::from("test"),
        parameter: vec![],
        returns_identifier: available_entities.first().unwrap().name,
    })
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
                },
            }
        } else {
            panic!();
        }
    }
}
*/
