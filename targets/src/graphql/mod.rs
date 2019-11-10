use ast::Multiplicity::{Concrete, Optional, UnderUpper};
use ast::{Entity, Namespace};
use generator::{Printer, Transform};
use parser::parse_from_file;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Deref;
use util::error::{PakError, PakResult};
use util::target::Target;
use util::Model;

pub struct GraphQLTarget {}
impl Target for GraphQLTarget {
    fn name(&self) -> &'static str { "graphql" }

    fn generate_from(model: Model) -> Result<(), PakError> {
        let namespace = parse_from_file(Path::from(model.name))?;
        let serialized = Schema::transform(namespace).serialize()?;
        let schema_file = Path::from("./schema.graphqls");
        let mut file = OpenOptions::new().write(true).create(true).open(schema_file)?;
        let res = file.write_all(serialized.as_bytes());
        Ok(())
    }
}

struct Schema {
    types: Vec<Type>,
}

struct Type {
    name: String,
    fields: Vec<Field>,
}

struct Field {
    name: String,
    typ: String,
}

impl Transform<Namespace> for Schema {
    fn transform(model: Namespace) -> Self {
        Schema { types: model.entities.iter().map(|e| Type::transform(Entity::from(e))).collect() }
    }
}

impl Transform<Entity> for Type {
    fn transform(model: Entity) -> Self {
        let mut fields = vec![];
        for attr in model.attributes {
            let mut typ = String::new();
            let is_nullable: bool = match attr.multiplicity {
                Optional => true,
                Concrete(num) => num == 0,
                UnderUpper(under, _upper) => under == 0,
                _ => false,
            };

            let mut is_list = false;
            if let UnderUpper(_, _) = attr.multiplicity {
                is_list = true;
                typ += "[";
            }
            typ += attr.entity_identifier.as_str();

            if is_list {
                typ += "]";
            }

            if !is_nullable {
                typ += "!";
            }
            fields.push(Field { name: attr.name, typ })
        }
        Type { name: model.name, fields }
    }
}

impl Printer for Field {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += self.name.as_str() + ": " + self.typ.as_str();
        Ok(output)
    }
}

impl Printer for Type {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += "type ";
        output += self.name.as_str();
        output += " {\n";
        for field in self.fields {
            output += field.serialize();
        }
        output += "}\n";
        Ok(output)
    }
}

impl Printer for Schema {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += "schema {\n";
        for t in self.types {
            output += t.serialize();
        }
        output += "}\n";
        Ok(output)
    }
}
