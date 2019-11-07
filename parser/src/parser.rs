use crate::error::ParserError;
use crate::pesten::{Parsable, Rule};
use crate::ParserResult;
use ast::{Attribute, Entity, Identifier, Namespace, Operation};
use pest::iterators::Pair;

impl Parsable for Namespace {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let mut entities = vec![];
        let identifier = String::from(
            inner_pairs.next().expect("Namespace should always have an identifier.").as_str(),
        );

        for inner_pair in inner_pairs {
            match inner_pair.as_rule() {
                Rule::entity => entities.push(Entity::from_pest(inner_pair)?),
                other => return Err(ParserError::InvalidRule(other)),
            }
        }
        let namespace = ast::Namespace { identifier, entities };

        Ok(namespace)
    }
}

impl Parsable for Entity {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let mut attributes: Vec<Attribute> = vec![];
        let mut operations: Vec<Operation> = vec![];
        let mut name = String::from(
            inner_pairs.next().expect("Entity should always have a parent identifier").as_str(),
        );
        let mut parent_identifier: Option<Identifier> = None;

        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::parententityname => {
                    parent_identifier = Some(String::from(pair.as_str()));
                },
                Rule::feature => {
                    let inner_feature = pair.into_inner();
                    for feature_pair in inner_feature {
                        match feature_pair.as_rule() {
                            Rule::attribute => attributes.push(Attribute::from_pest(feature_pair)?), /* TODO add self to available_entities? */
                            Rule::operation => operations.push(Operation::from_pest(feature_pair)?),
                            other => return Err(ParserError::InvalidRule(other)),
                        }
                    }
                },
                other => return Err(ParserError::InvalidRule(other)),
            }
        }

        let parsed = Entity { name, attributes, operations, parent_identifier };
        Ok(parsed)
    }
}

impl Parsable for Attribute {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        unimplemented!();
    }
}

impl Parsable for Operation {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        unimplemented!();
    }
}

#[cfg(tests)]
mod tests {
    use crate::error::ParserError;
    use crate::parse;
    use crate::pesten::Parsable;
    use ast::Namespace;
    use std::fs;
    use std::path::Path;

    #[test]
    fn parse_namespace_ok() {
        let code = "namespace.is.lit {}";
        let expected = Namespace { identifier: String::from("namespace.is.lit"), entities: vec![] };
        assert_eq!(Namespace::pest_parse(Rule::namespace, code), expected);
    }

    #[test]
    fn parse_namespace_fail() {
        let code = "namespace.is.lit";
        assert_eq!(Namespace::pest_parse(Rule::namespace, code), ParserError::ParsingError);
    }

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
