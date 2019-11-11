use ast::Multiplicity::{Concrete, Optional, UnderUpper};
use ast::Number::Discrete;
use ast::{Entity, Namespace};
use generator::{Printer, Transform};
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
        let serialized = Schema::transform(&namespace).serialize()?;
        let file_structure = FileStructure::Dir("graphql".to_owned(), vec![FileStructure::File(
            "schema.graphqls".to_owned(),
            serialized,
        )]);
        Ok(file_structure)
    }
}

#[derive(Debug, Clone)]
struct Schema {
    types: Vec<Type>,
}

#[derive(Debug, Clone)]
struct Type {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    typ: String,
}

impl Transform<Namespace> for Schema {
    fn transform(model: &Namespace) -> Self {
        Schema { types: model.entities.iter().map(|e| Type::transform(e)).collect() }
    }
}

impl Transform<Entity> for Type {
    fn transform(model: &Entity) -> Self {
        let mut fields = vec![];
        for attr in model.attributes.clone() {
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
            typ += attr.entity_identifier.as_str();

            if is_list {
                typ += "]";
            }

            if !is_nullable {
                typ += "!";
            }
            fields.push(Field { name: attr.name, typ })
        }
        Type { name: model.name.clone(), fields }
    }
}

impl Printer for Field {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += self.name.as_str();
        output += ": ";
        output += self.typ.as_str();
        Ok(output)
    }
}

impl Printer for Type {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += "type ";
        output += self.name.as_str();
        output += " {\n";
        for field in self.fields.clone() {
            output += field.serialize()?.as_str();
        }
        output += "}\n";
        Ok(output)
    }
}

impl Printer for Schema {
    fn serialize(&self) -> PakResult<String> {
        let mut output = String::new();
        output += "schema {\n";
        for t in self.types.clone() {
            output += t.serialize()?.as_str();
        }
        output += "}\n";
        Ok(output)
    }
}
