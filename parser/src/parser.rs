use crate::dangling::{
    DanglingAttribute, DanglingOperation, DanglingParameter, DanglingStructure, Undangle,
};
use crate::error::ParserError;
use crate::pesten::{Parsable, Rule};
use crate::ParserResult;
use ast::Entity::Scalar as EScalar;
use ast::Scalar;
use ast::{Entity, Identifier, Multiplicity, Namespace, Number};
use pest::iterators::Pair;
use std::rc::Rc;

lazy_static! {
    static ref DEFAULT_ENTITIES: Vec<Entity> = {
        vec![
            EScalar(Scalar::String),
            EScalar(Scalar::Boolean),
            EScalar(Scalar::Character),
            EScalar(Scalar::Double),
            EScalar(Scalar::Integer),
        ]
    };
}

impl Parsable for Namespace {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let identifier = String::from(
            inner_pairs.next().expect("Namespace should always have an identifier.").as_str(),
        );

        let mut namespace = Namespace::new(identifier);

        for e in DEFAULT_ENTITIES {
            namespace.add_entity(Rc::new(e));
        }

        for inner_pair in inner_pairs {
            match inner_pair.as_rule() {
                Rule::entity => {
                    let struc = DanglingStructure::from_pest(inner_pair)?;
                    let entity = Entity::Structure(struc.undangle(&namespace)?);
                    namespace.add_entity(Rc::new(entity));
                },
                other => return Err(ParserError::InvalidRule(other)),
            }
        }

        Ok(namespace)
    }
}

impl Parsable for DanglingStructure {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let mut attributes: Vec<DanglingAttribute> = vec![];
        let mut operations: Vec<DanglingOperation> = vec![];
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
                            Rule::attribute => {
                                attributes.push(DanglingAttribute::from_pest(feature_pair)?)
                            },
                            Rule::operation => {
                                operations.push(DanglingOperation::from_pest(feature_pair)?)
                            },
                            other => return Err(ParserError::InvalidRule(other)),
                        }
                    }
                },
                other => return Err(ParserError::InvalidRule(other)),
            }
        }

        let parsed = DanglingStructure { name, attributes, operations, parent: parent_identifier };
        Ok(parsed)
    }
}

impl Parsable for DanglingAttribute {
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

        Ok(DanglingAttribute { name, entity: entity_identifier, multiplicity })
    }
}

impl Parsable for DanglingOperation {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let name = String::from(inner_pairs.next().expect("Operation must have a name.").as_str());
        let mut parameter = vec![];

        let next = inner_pairs.next().unwrap();
        if next.as_rule() == Rule::parameterlist {
            for parameter_pair in next.into_inner() {
                parameter.push(DanglingParameter::from_pest(parameter_pair)?);
            }
        }

        let returns_identifier =
            String::from(inner_pairs.next().expect("This should be the return type.").as_str());

        Ok(DanglingOperation { name, parameter, returns: returns_identifier })
    }
}

impl Parsable for DanglingParameter {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let name =
            String::from(inner_pairs.next().expect("Parameter should have a name.").as_str());
        let entity_identifier = String::from(
            inner_pairs.next().expect("Parameter should have a return type.").as_str(),
        );

        Ok(DanglingParameter { name, entity: entity_identifier })
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
