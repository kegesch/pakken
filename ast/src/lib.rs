pub struct Namespace {
    identifier: String,
    entities: Vec<Entity>,
}

pub struct Entity {
    name: String,
    parent: Option<Box<Entity>>,
    attributes: Vec<Attribute>,
    operations: Vec<Operation>,
}

pub enum Multiplicity {
    Concrete(i8),
    UnderUpper(i8, i8),
    Single,
    Optional,
}

pub struct Attribute {
    name: String,
    entity: Entity,
    multiplicity: Multiplicity,
}

pub struct Parameter {
    name: String,
    entity: Entity,
}

pub struct Operation {
    name: String,
    returns: Entity,
    parameter: Vec<Parameter>,
}
