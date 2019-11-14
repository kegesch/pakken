use ast::Multiplicity::{Concrete, Optional, UnderUpper};
use ast::Number::Discrete;
use ast::{Entity, Identifying, Namespace};
use generator::{Buffer, Printer, Transform};
use parser::parse_from_file;
use util::error::PakResult;
use util::target::Target;
use util::{FileStructure, Model};

#[derive(Default)]
pub struct GraphQLTarget {}
impl Target for GraphQLTarget {
    fn name(&self) -> &'static str { "graphql" }

    fn generate_from(&self, model: Model) -> PakResult<FileStructure> {
        let namespace = parse_from_file(model.path.as_path())?;
        let serialized = Schema::transform(&namespace).serialize(Buffer::default()).flush();
        let file_structure = FileStructure::Dir("graphql".to_owned(), vec![FileStructure::File(
            "schema.graphqls".to_owned(),
            serialized,
        )]);
        Ok(file_structure)
    }
}

#[derive(Debug, Clone)]
struct Schema {
    types: Vec<Typed>,
}

#[derive(Debug, Clone)]
struct Type {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
enum Typed {
    Type(Type),
    None,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    typ: String,
}

impl Transform<Namespace> for Schema {
    fn transform(model: &Namespace) -> Self {
        Schema { types: model.entities.iter().map(|e| Typed::transform(e)).collect() }
    }
}

impl Transform<Entity> for Typed {
    fn transform(model: &Entity) -> Self {
        let mut fields = vec![];
        if let Entity::Structure(struc) = model {
            for attr in struc.attributes.clone() {
                let mut typ = String::new();
                let is_nullable: bool = match attr.multiplicity {
                    Optional => true,
                    Concrete(Discrete(num)) => num == 0,
                    UnderUpper(Discrete(under), _upper) => under == 0,
                    _ => false,
                };

                let mut is_list = false;
                if let UnderUpper(_, _) = attr.multiplicity {
                    is_list = true;
                    typ += "[";
                }
                typ += attr.entity.identifier().as_str();

                if is_list {
                    typ += "]";
                }

                if !is_nullable {
                    typ += "!";
                }
                fields.push(Field { name: attr.name, typ })
            }
            Typed::Type(Type { name: struc.name.clone(), fields })
        } else {
            Typed::None
        }
    }
}

impl Printer for Field {
    fn serialize(&self, mut buffer: Buffer) -> Buffer {
        buffer += self.name.as_str();
        buffer += ": ";
        buffer += self.typ.as_str();
        buffer
    }
}

impl Printer for Typed {
    fn serialize(&self, mut buffer: Buffer) -> Buffer {
        match self {
            Typed::Type(t) => buffer = t.serialize(buffer),
            Typed::None => (),
        }
        buffer
    }
}

impl Printer for Type {
    fn serialize(&self, mut buffer: Buffer) -> Buffer {
        buffer += "type ";
        buffer += self.name.as_str();
        buffer += " {";
        buffer.indent();
        buffer.new_line();
        for field in self.fields.clone() {
            buffer = field.serialize(buffer);
            buffer += ",";
            buffer.new_line();
        }
        buffer.unindent();
        buffer.new_line();
        buffer += "}";
        buffer.new_line();
        buffer
    }
}

impl Printer for Schema {
    fn serialize(&self, mut buffer: Buffer) -> Buffer {
        buffer += "schema {";
        buffer.new_line();
        buffer += "}";
        buffer.new_line();

        for t in self.types.clone() {
            buffer = t.serialize(buffer);
            buffer.new_line();
        }
        buffer
    }
}
