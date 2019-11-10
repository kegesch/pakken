pub type Identifier = String;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub identifier: Identifier,
    pub entities: Vec<Entity>,
}

impl Namespace {
    pub fn resolve_entity(&self, identifier: &Identifier) -> Option<&Entity> {
        self.entities.iter().find(|e: &&Entity| e.name.eq(identifier))
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub parent_identifier: Option<Identifier>, /* thought about referencing an entity here, but rust doesn't allow struct owning a value and referencing it in itself */
    pub attributes: Vec<Attribute>,
    pub operations: Vec<Operation>,
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
    pub entity_identifier: Identifier, /* thought about referencing an entity here, but rust doesn't allow struct owning a value and referencing it in itself */
    pub multiplicity: Multiplicity,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub entity_identifier: Identifier, /* thought about referencing an entity here, but rust doesn't allow struct owning a value and referencing it in itself */
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub name: String,
    pub returns_identifier: Identifier, /* thought about referencing an entity here, but rust doesn't allow struct owning a value and referencing it in itself */
    pub parameter: Vec<Parameter>,
}
