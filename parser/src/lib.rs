extern crate pest;
extern crate pest_derive;

pub mod error;
pub mod parser;
pub mod pesten;

use crate::error::ParserError;
use crate::pesten::Parsable;
use ast::*;

type ParserResult<T> = std::result::Result<T, ParserError>;

pub fn parse(code: &str) -> ParserResult<Namespace> {
    Namespace::pest_parse(pesten::Rule::namespace, code)
}

#[cfg(tests)]
mod tests {
    use crate::parse;
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
            match parse(code.as_str()) {
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

/*
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

*/
