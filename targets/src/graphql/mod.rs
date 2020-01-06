use ast::Multiplicity::{Concrete, Optional, UnderUpper};
use ast::Number::Discrete;
use ast::{Entity, Identifying, Namespace};
use generator::Transform;
use parser::parse_from_file;
use util::buffer::Buffer;
use util::code::{CodePage, GeneratedCode};
use util::error::PakResult;
use util::filestructure::FileStructure;
use util::target::Target;
use util::{Generate, Model};

#[derive(Default)]
pub struct GraphQLTarget {}
impl Target for GraphQLTarget {
    fn name(&self) -> &'static str { "graphql" }

    fn generate_from(&self, model: Model) -> PakResult<FileStructure> {
        let namespace = parse_from_file(model.path.as_path())?;
        let transformed = Document::transform(&namespace);
        let schema = transformed.generate();
        let file_structure = FileStructure::Dir("graphql".to_owned(), vec![FileStructure::File(
            "schema.graphqls".to_owned(),
            schema,
        )]);
        Ok(file_structure)
    }
}

#[derive(Debug, Clone)]
struct Document {
    types: Vec<Typed>,
    schema: Schema,
}

#[derive(Debug, Clone)]
struct Query {
    queries: Vec<String>,
}

#[derive(Debug, Clone)]
struct Mutation {
    mutations: Vec<String>,
}

#[derive(Debug, Clone)]
struct Schema {
    query: Query,
    mutation: Mutation,
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

impl Transform<Namespace> for Document {
    fn transform(model: &Namespace) -> Self {
        let types: Vec<Typed> = model.entities.iter().map(|e| Typed::transform(e)).collect();
        let schema = Schema::transform(&types);
        Document { types, schema }
    }
}

impl Transform<Vec<Typed>> for Schema {
    fn transform(model: &Vec<Typed>) -> Self {
        let query = Query::transform(model);
        let mutation = Mutation::transform(model);
        Schema { query, mutation }
    }
}

impl Transform<Vec<Typed>> for Query {
    fn transform(model: &Vec<Typed>) -> Self {
        let mut queries: Vec<String> = vec![];
        for t in model {
            if let Typed::Type(typ) = t {
                // TODO one for id,
                let query = format!("query{}: [{}!]", &typ.name, &typ.name);
                queries.push(query);
            }
        }
        Query { queries }
    }
}

impl Transform<Vec<Typed>> for Mutation {
    fn transform(model: &Vec<Typed>) -> Self {
        let mut mutations: Vec<String> = vec![];
        for t in model {
            if let Typed::Type(typ) = t {
                let mut params = vec![];
                for attr in typ.fields.clone() {
                    let mut param = String::new();
                    param.push_str(attr.name.as_str());
                    param.push_str(": ");
                    param.push_str(attr.typ.as_str());
                    params.push(param);
                }

                // TODO handle id
                let query = format!("create{}({}): [{}!]", &typ.name, params.join(", "), &typ.name);
                mutations.push(query);
            }
        }
        Mutation { mutations }
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

impl Field {
    fn generate(&self) -> String {
        let mut buffer = Buffer::default();
        buffer += self.name.as_str();
        buffer += ": ";
        buffer += self.typ.as_str();

        buffer.flush()
    }
}

impl Typed {
    fn generate(&self) -> Option<GeneratedCode> {
        match self {
            Typed::Type(t) => Some(t.generate()),
            Typed::None => None,
        }
    }
}

impl Generate for Type {
    fn generate(&self) -> GeneratedCode {
        let mut buffer = Buffer::default();
        buffer += "type ";
        buffer += self.name.as_str();
        buffer += " {";
        buffer.indent();
        for field in self.fields.clone() {
            buffer.new_line();
            buffer += field.generate().as_str();
            buffer += ",";
        }
        buffer.unindent();
        buffer.new_line();
        buffer += "}";

        GeneratedCode { code: buffer.flush(), id: self.name.clone() }
    }
}

impl Generate for Schema {
    fn generate(&self) -> GeneratedCode {
        let mut buffer = Buffer::default();
        buffer += "schema {";
        buffer.indent();
        buffer.new_line();
        buffer += "query: Query,";
        buffer.new_line();
        buffer += "mutation: Mutation,";
        buffer.unindent();
        buffer.new_line();
        buffer += "}";
        buffer.new_line();
        buffer.new_line();
        buffer += self.mutation.generate().as_str();
        buffer.new_line();
        buffer += self.query.generate().as_str();

        GeneratedCode { code: buffer.flush(), id: "schema".to_string() }
    }
}

impl Query {
    fn generate(&self) -> String {
        let mut buffer = Buffer::default();
        buffer += "type Query {";
        buffer.indent();
        for query in self.queries.clone() {
            buffer.new_line();
            buffer += query.as_str();
            buffer += ",";
        }
        buffer.unindent();
        buffer.new_line();
        buffer += "}";

        buffer.flush()
    }
}

impl Mutation {
    fn generate(&self) -> String {
        let mut buffer = Buffer::default();
        buffer += "type Mutation {";
        buffer.indent();
        for query in self.mutations.clone() {
            buffer.new_line();
            buffer += query.as_str();
            buffer += ",";
        }
        buffer.unindent();
        buffer.new_line();
        buffer += "}";
        buffer.flush()
    }
}

impl Document {
    fn generate(&self) -> CodePage {
        let mut codepage = CodePage::default("#");

        for typ in self.types.clone() {
            if let Some(generated) = typ.generate() {
                codepage.add(generated.to_fragment());
            }
        }
        codepage.add(self.schema.generate().to_fragment());
        codepage
    }
}
