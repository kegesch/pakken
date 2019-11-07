use crate::error::ParserError;
use crate::pesten::{Parsable, Rule};
use crate::ParserResult;
use ast::{Attribute, Entity, Identifier, Multiplicity, Namespace, Number, Operation};
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
        let name = String::from(
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
        let mut inner_pairs = pair.into_inner();

        let name =
            String::from(inner_pairs.next().expect("Attribute must always have a name.").as_str());
        let entity_identifier =
            String::from(inner_pairs.next().expect("Attribute must always have type.").as_str());
        let multiplicity = match inner_pairs.next() {
            Some(pair) => Multiplicity::from_pest(pair)?,
            None => Multiplicity::Single,
        };

        Ok(Attribute { name, entity_identifier, multiplicity })
    }
}

impl Parsable for Operation {
    fn from_pest(_pair: Pair<Rule>) -> ParserResult<Self> {
        Ok(Operation {
            name: String::from("test"),
            parameter: vec![],
            returns_identifier: String::from("reuturntype"),
        })
    }
}

impl Parsable for Multiplicity {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        if let Some(multi_pair) = pair.into_inner().next() {
            match multi_pair.as_rule() {
                Rule::listmult => {
                    if multi_pair.as_str().eq("+") {
                        Ok(Multiplicity::UnderUpper(Number::Discrete(1), Number::Infinity))
                    } else if multi_pair.as_str().eq("*") {
                        Ok(Multiplicity::UnderUpper(Number::Discrete(0), Number::Infinity))
                    } else {
                        Err(ParserError::Unhandled)
                    }
                },
                Rule::multimult => {
                    let mut inner = multi_pair.into_inner();
                    let under = inner
                        .next()
                        .expect("There must be an under bound.")
                        .as_str()
                        .parse()
                        .unwrap();
                    let upper = inner
                        .next()
                        .expect("There must be an upper bound.")
                        .as_str()
                        .parse()
                        .unwrap();
                    if under == 0 && upper == 1 {
                        Ok(Multiplicity::Optional)
                    } else {
                        Ok(Multiplicity::UnderUpper(
                            Number::Discrete(under),
                            Number::Discrete(upper),
                        ))
                    }
                },
                Rule::singlemult => {
                    let parsed = multi_pair.as_str().parse().unwrap();
                    if parsed == 1 {
                        Ok(Multiplicity::Single)
                    } else {
                        Ok(Multiplicity::Concrete(Number::Discrete(parsed)))
                    }
                },
                Rule::optionalmult => Ok(Multiplicity::Optional),
                other => Err(ParserError::InvalidRule(other)),
            }
        } else {
            Ok(Multiplicity::Single)
        }
    }
}

#[cfg(tests)]
mod tests {
    use crate::error::ParserError;
    use crate::pesten::Parsable;
    use ast::Namespace;

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
}
