use ast::Namespace;
use parser::parse;

fn main() {
    let code = "io.test {";
    println!("Parsing! {:?}", code);
    println!("Result: {:?}", parse(code));
}
