use ast::Entity;
use generator::{Printer, Transform};
use util::error::PakError;

struct Type {
    name: String,
}

impl Transform<Entity> for Type {
    fn transform(model: Entity) -> Self { Type { name: model.name } }
}

impl Printer for Type {
    fn serialize(&self) -> Result<String, PakError> {
        let mut output = String::new();
        output += "type ";
        output += self.name.as_str();
        output += " {\n";
        output += "}";
        Ok(output)
    }
}
