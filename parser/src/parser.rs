use crate::dangling::{
    DanglingAttribute, DanglingOperation, DanglingParameter, DanglingStructure, Undangle,
};
use crate::error::ParserError;
use crate::pesten::{Parsable, Rule};
use crate::ParserResult;
use ast::Entity::Scalar as EScalar;
use ast::{Entity, Identifier, Multiplicity, Namespace, Number};
use ast::{Enum, Scalar};
use pest::iterators::Pair;
use std::rc::Rc;

impl Parsable for Namespace {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let identifier = String::from(
            inner_pairs.next().expect("Namespace should always have an identifier.").as_str(),
        );

        let mut namespace = Namespace::new(identifier);

        namespace.add_entity(Rc::new(EScalar(Scalar::String)));
        namespace.add_entity(Rc::new(EScalar(Scalar::Boolean)));
        namespace.add_entity(Rc::new(EScalar(Scalar::Character)));
        namespace.add_entity(Rc::new(EScalar(Scalar::Double)));
        namespace.add_entity(Rc::new(EScalar(Scalar::Integer)));

        for inner_pair in inner_pairs {
            match inner_pair.as_rule() {
                Rule::entitytype => {
                    let entity_type =
                        inner_pair.into_inner().next().expect("There must be a type of entity.");
                    match entity_type.as_rule() {
                        Rule::entity => {
                            let struc = DanglingStructure::from_pest(entity_type)?;
                            let entity = Entity::Structure(struc.undangle(&namespace)?);
                            namespace.add_entity(Rc::new(entity));
                        },
                        Rule::enumeration => {
                            let enumeration = Entity::Enum(Enum::from_pest(entity_type)?);
                            namespace.add_entity(Rc::new(enumeration));
                        },
                        other => return Err(ParserError::InvalidRule(other)),
                    }
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
        let returns_identifier: Option<String>;
        if let Some(returns) = inner_pairs.next() {
            returns_identifier = Some(returns.as_str().to_string());
        } else {
            returns_identifier = None;
        }

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

impl Parsable for Enum {
    fn from_pest(pair: Pair<Rule>) -> ParserResult<Self> {
        let mut inner_pairs = pair.into_inner();
        let name = inner_pairs.next().expect("Enumeration should have a name.").as_str();
        let mut members = vec![];
        for pair in inner_pairs {
            let mut member_pairs = pair.into_inner();
            let member_name =
                member_pairs.next().expect("Enumeration member should have a name.").as_str();
            let member_value = member_pairs.next().map(|i| i.as_str().parse::<usize>().unwrap());
            members.push((member_name.to_string(), member_value));
        }
        Ok(Enum { identifier: name.to_string(), values: members })
    }
}

#[cfg(test)]
mod tests {
    use crate::dangling::DanglingStructure;
    use crate::error::ParserError;
    use crate::pesten::{PakkenRule, Parsable};
    use ast::Namespace;

    #[test]
    fn parse_namespace_ok() {
        let code = "namespace.is.lit {}";
        let expected = Namespace { identifier: String::from("namespace.is.lit"), entities: vec![] };
        let parsed =
            Namespace::pest_parse(PakkenRule::namespace, code).expect("Should have parsed");
        assert_eq!(parsed.identifier, expected.identifier);
    }

    #[test]
    fn parse_namespace_fail() {
        let code = "namespace.is.lit";
        let parsed = Namespace::pest_parse(PakkenRule::namespace, code);
        if let ParserError::ParsingError(_, _) = parsed.unwrap_err() {
            assert_eq!(true, true);
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn parse_structure() {
        let code = "Name { attribute: String, operation() }";
        let expected = DanglingStructure {
            name: "Name".to_string(),
            parent: None,
            attributes: vec![],
            operations: vec![],
        };

        let parsed = DanglingStructure::pest_parse(PakkenRule::entitytype, code)
            .expect("Should have parsed");
    }
}
