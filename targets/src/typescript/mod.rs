use ast::{Entity, Identifying, Namespace, Scalar, Structure};
use generator::Transform;
use parser::parse_from_file;
use util::buffer::Buffer;
use util::code::{CodePage, GeneratedCode};
use util::error::PakResult;
use util::filestructure::FileStructure;
use util::target::Target;
use util::{Generate, Model};

#[derive(Default)]
pub struct TypeScriptTarget {}
impl Target for TypeScriptTarget {
    fn name(&self) -> &'static str { "typescript" }

    fn generate_from(&self, model: Model) -> PakResult<FileStructure> {
        let namespace = parse_from_file(model.path.as_path())?;
        let transformed = Declaration::transform(&namespace);
        let schema = transformed.generate();
        let file_structure =
            FileStructure::Dir("typescript".to_owned(), vec![FileStructure::File(
                "global.d.ts".to_owned(),
                schema,
            )]);
        Ok(file_structure)
    }
}

#[derive(Debug)]
struct Declaration {
    typed: Vec<Typed>,
}

#[derive(Debug)]
enum Typed {
    Class(Class),
    Interface(Interface),
    Scalar(String),
    Vec(Vec<Typed>),
    None,
}

impl Typed {
    fn identifier(&self) -> String {
        match self {
            Typed::Class(class) => class.name.clone(),
            Typed::Scalar(scalar) => scalar.clone(),
            Typed::Interface(interface) => interface.name.clone(),
            Typed::Vec(vec) => vec.first().expect("Should have at least one member!").identifier(),
            _ => panic!("Could not get identifier for {:?}", self),
        }
    }

    fn is_some(&self) -> bool {
        match self {
            Typed::Class(_) => true,
            Typed::Scalar(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Interface {
    name: String,
    extends: Option<Vec<String>>,
    attributes: Vec<String>,
    operations: Vec<String>,
}

#[derive(Debug)]
struct Class {
    name: String,
    extends: Option<String>,
    implements: Option<Vec<String>>,
    attributes: Vec<String>,
    operations: Vec<String>,
}

impl Transform<Entity> for Typed {
    fn transform(model: &Entity) -> Self {
        match model {
            Entity::Structure(struc) => {
                let mut vec = vec![];
                vec.push(Typed::Class(Class::transform(struc)));
                vec.push(Typed::Interface(Interface::transform(struc)));
                Typed::Vec(vec)
            },
            Entity::Scalar(scalar) => match scalar {
                Scalar::String => Typed::Scalar("string".to_owned()),
                Scalar::Double => Typed::Scalar("number".to_owned()),
                Scalar::Integer => Typed::Scalar("number".to_owned()),
                Scalar::Character => Typed::Scalar("char".to_owned()),
                Scalar::Boolean => Typed::Scalar("bool".to_owned()),
            },
            _ => Typed::None,
        }
    }
}

impl Declaration {
    fn generate(&self) -> CodePage {
        let mut fragments = vec![];
        for tp in &self.typed {
            if let Typed::Class(class) = tp {
                fragments.push(class.generate().to_fragment());
            } else if let Typed::Interface(interface) = tp {
                fragments.push(interface.generate().to_fragment())
            }
        }
        CodePage { comment_string: "//", fragments }
    }
}

impl Generate for Class {
    fn generate(&self) -> GeneratedCode {
        let mut buf = Buffer::default();
        buf += "export class ";
        buf += self.name.as_str();
        if let Some(extends) = &self.extends {
            buf += " extends ";
            buf += extends.as_str();
        }
        if let Some(implements) = &self.implements {
            buf += " implements ";
            buf += implements.iter().map(|s| &**s).collect::<Vec<&str>>().join(", ").as_str();
        }
        buf += " {";
        buf.indent();
        for attr in &self.attributes {
            buf.new_line();
            buf += attr.as_str();
            buf += ";";
        }
        for op in &self.operations {
            buf.new_line();
            buf += op.as_str();
        }
        buf.unindent();
        buf.new_line();
        buf += "}";

        GeneratedCode { id: self.name.clone(), code: buf.flush() }
    }
}

impl Generate for Interface {
    fn generate(&self) -> GeneratedCode {
        let mut buf = Buffer::default();
        buf += "export interface ";
        buf += self.name.as_str();
        if let Some(extends) = &self.extends {
            buf += " extends ";
            buf += extends.iter().map(|s| &**s).collect::<Vec<&str>>().join(", ").as_str();
        }
        buf += " {";
        buf.indent();
        for attr in &self.attributes {
            buf.new_line();
            buf += attr.as_str();
            buf += ";";
        }
        for op in &self.operations {
            buf.new_line();
            buf += op.as_str();
        }
        buf.unindent();
        buf.new_line();
        buf += "}";

        GeneratedCode { id: self.name.clone(), code: buf.flush() }
    }
}

impl Transform<Namespace> for Declaration {
    fn transform(model: &Namespace) -> Self {
        let mut flattened = vec![];
        //let typed: Vec<Typed> = model.entities.iter().map(|e| Typed::transform(e)).collect();
        for entity in &model.entities {
            let tp = Typed::transform(&entity);
            if let Typed::Vec(vec) = tp {
                for tp2 in vec {
                    flattened.push(tp2);
                }
            } else {
                flattened.push(tp);
            }
        }

        Declaration { typed: flattened }
    }
}

impl Transform<Structure> for Interface {
    fn transform(model: &Structure) -> Self {
        let mut ops: Vec<String> = vec![];
        for op in &model.operations {
            let mut buf = Buffer::default();
            buf += op.name.as_str();
            buf += "(";
            for (index, param) in op.parameter.iter().enumerate() {
                buf += param.name.as_str();
                buf += ": ";
                buf += Typed::transform(param.entity.as_ref()).identifier().as_str();
                if index < op.parameter.len() - 1 {
                    buf += ",";
                }
            }
            buf += ");";
            ops.push(buf.flush());
        }
        let mut attrs = vec![];
        for attr in &model.attributes {
            let mut buf = Buffer::default();
            buf += attr.name.as_str();
            buf += ": ";
            buf += attr.entity.identifier().as_str();
            attrs.push(buf.flush());
        }
        Interface {
            name: String::from("I") + &model.name,
            operations: ops,
            attributes: attrs,
            extends: None,
        }
    }
}

impl Transform<Structure> for Class {
    fn transform(model: &Structure) -> Self {
        let mut ops: Vec<String> = vec![];
        for op in &model.operations {
            let mut buf = Buffer::default();
            buf += "public ";
            buf += op.name.as_str();
            buf += "(";
            for (index, param) in op.parameter.iter().enumerate() {
                buf += param.name.as_str();
                buf += ": ";
                buf += Typed::transform(param.entity.as_ref()).identifier().as_str();
                if index < op.parameter.len() - 1 {
                    buf += ",";
                }
            }
            buf += ")";
            if let Some(ent) = &op.returns {
                let typed = Typed::transform(ent);
                if typed.is_some() {
                    buf += ": ";
                    buf += typed.identifier().as_str();
                }
            }
            buf += " {";
            buf.indent();
            buf.new_line();
            buf += "// TODO implement";
            if op.returns.is_some() {
                buf.new_line();
                buf += "return undefined;";
            }
            buf.unindent();
            buf.new_line();
            buf += "}";
            buf.new_line();
            ops.push(buf.flush());
        }

        let mut attrs = vec![];
        for attr in &model.attributes {
            let mut buf = Buffer::default();
            buf += "public ";
            buf += attr.name.as_str();
            buf += ": ";
            buf += attr.entity.identifier().as_str();
            attrs.push(buf.flush());
        }

        Class {
            name: model.identifier(),
            extends: model.parent.as_ref().map(|p| p.identifier()),
            operations: ops,
            attributes: attrs,
            implements: Some(vec![String::from("I") + &model.name]),
        }
    }
}
