use crate::error::ParserError::InvalidEntity;
use crate::ParserResult;
use ast::{Attribute, Entity, Multiplicity, Namespace, Operation, Parameter, Structure};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct DanglingStructure {
    pub name: String,
    pub parent: Option<String>,
    pub attributes: Vec<DanglingAttribute>,
    pub operations: Vec<DanglingOperation>,
}

#[derive(Debug, Clone)]
pub struct DanglingAttribute {
    pub name: String,
    pub entity: String,
    pub multiplicity: Multiplicity,
}

#[derive(Debug, Clone)]
pub struct DanglingParameter {
    pub name: String,
    pub entity: String,
}

#[derive(Debug, Clone)]
pub struct DanglingOperation {
    pub name: String,
    pub returns: String,
    pub parameter: Vec<DanglingParameter>,
}

pub trait Undangle {
    type Undangled;

    fn undangle(&self, namespace: &Namespace) -> ParserResult<Self::Undangled>;
    fn resolve(identifier: &str, namespace: &Namespace) -> ParserResult<Rc<Entity>> {
        if let Some(entity) = namespace.find_entity(identifier.to_owned()) {
            Ok(entity)
        } else {
            Err(InvalidEntity(identifier.to_owned()))
        }
    }
}

impl Undangle for DanglingParameter {
    type Undangled = Parameter;

    fn undangle(&self, namespace: &Namespace) -> ParserResult<Self::Undangled> {
        let result: ParserResult<Rc<Entity>> =
            Self::resolve(self.entity.as_str(), namespace) as ParserResult<Rc<Entity>>;
        let resolved = result?;
        Ok(Parameter { name: self.name.clone(), entity: resolved })
    }
}

impl Undangle for DanglingOperation {
    type Undangled = Operation;

    fn undangle(&self, namespace: &Namespace) -> ParserResult<Self::Undangled> {
        let result: ParserResult<Rc<Entity>> =
            Self::resolve(self.returns.as_str(), namespace) as ParserResult<Rc<Entity>>;
        let resolved = result?;
        let mut undangled_parameters = vec![];
        for param in self.parameter.clone() {
            undangled_parameters.push(param.undangle(namespace)?);
        }
        Ok(Operation {
            name: self.name.clone(),
            returns: resolved,
            parameter: undangled_parameters,
        })
    }
}

impl Undangle for DanglingAttribute {
    type Undangled = Attribute;

    fn undangle(&self, namespace: &Namespace) -> ParserResult<Self::Undangled> {
        let result: ParserResult<Rc<Entity>> =
            Self::resolve(self.entity.as_str(), namespace) as ParserResult<Rc<Entity>>;
        let resolved = result?;
        Ok(Attribute { name: self.name.clone(), entity: resolved, multiplicity: self.multiplicity })
    }
}

impl Undangle for DanglingStructure {
    type Undangled = Structure;

    fn undangle(&self, namespace: &Namespace) -> ParserResult<Self::Undangled> {
        let parent = match self.parent.clone() {
            Some(parent) => {
                let result = Self::resolve(parent.as_str(), namespace);
                let resolved = result?;
                Some(resolved)
            },
            None => None,
        };
        let mut undangled_attributes = vec![];
        for attr in self.attributes.clone() {
            undangled_attributes.push(attr.undangle(namespace)?);
        }
        let mut undangled_operations = vec![];
        for op in self.operations.clone() {
            undangled_operations.push(op.undangle(namespace)?);
        }

        Ok(Structure {
            name: self.name.clone(),
            attributes: undangled_attributes,
            operations: undangled_operations,
            parent,
        })
    }
}
