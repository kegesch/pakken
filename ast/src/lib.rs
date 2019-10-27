pub struct Namespace {
    pub identifier: String,
    pub entities: Vec<Entity>,
}

pub struct Entity {
    pub name: String,
    pub parent: Option<Box<Entity>>,
    pub attributes: Vec<Attribute>,
    pub operations: Vec<Operation>,
}

pub enum Multiplicity {
    Concrete(i8),
    UnderUpper(i8, i8),
    Single,
    Optional,
}

pub struct Attribute {
    pub name: String,
    pub entity: Entity,
    pub multiplicity: Multiplicity,
}

pub struct Parameter {
    pub name: String,
    pub entity: Entity,
}

pub struct Operation {
    pub name: String,
    pub returns: Entity,
    pub parameter: Vec<Parameter>,
}
