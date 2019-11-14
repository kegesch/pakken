use core::fmt;
use std::rc::Rc;

pub type Identifier = String;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub identifier: Identifier,
    pub entities: Vec<Rc<Entity>>,
}

impl Namespace {
    pub fn find_entity(&self, identifier: Identifier) -> Option<Rc<Entity>> {
        let found = self.entities.iter().find(|e: &&Rc<Entity>| e.identifier() == identifier);
        if let Some(entity) = found {
            Some(Rc::clone(entity))
        } else {
            None
        }
    }

    pub fn new(identifier: Identifier) -> Namespace { Namespace { identifier, entities: vec![] } }

    pub fn add_entity(&mut self, entity: Rc<Entity>) { self.entities.push(entity) }
}

pub trait Identifying {
    fn identifier(&self) -> Identifier;
}

#[derive(Debug, Clone, Copy)]
pub enum Scalar {
    String,
    Integer,
    Double,
    Boolean,
    Character,
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:?}", self) }
}

impl Identifying for Scalar {
    fn identifier(&self) -> String { self.to_string() }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub identifier: Identifier,
    pub values: (String, Option<usize>),
}

impl Identifying for Enum {
    fn identifier(&self) -> String { self.identifier.clone() }
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub name: String,
    pub parent: Option<Rc<Entity>>,
    pub attributes: Vec<Attribute>,
    pub operations: Vec<Operation>,
}

impl Identifying for Structure {
    fn identifier(&self) -> String { self.name.clone() }
}

#[derive(Debug, Clone)]
pub enum Entity {
    Structure(Structure),
    Scalar(Scalar),
    Enum(Enum),
}

impl Identifying for Entity {
    fn identifier(&self) -> String {
        match self {
            Entity::Structure(s) => s.identifier(),
            Entity::Scalar(s) => s.identifier(),
            Entity::Enum(e) => e.identifier(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Multiplicity {
    Concrete(Number),
    UnderUpper(Number, Number),
    Single,
    Optional,
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Discrete(usize),
    Infinity,
    NegativeInfinity,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub entity: Rc<Entity>,
    pub multiplicity: Multiplicity,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub entity: Rc<Entity>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub returns: Rc<Entity>,
    pub parameter: Vec<Parameter>,
}
