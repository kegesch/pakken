use parser::parse;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./parser/test/example.pakken");
    println!("{}", path.display());
    let file = fs::read_to_string(path.canonicalize().unwrap());
    println!("{}", path.canonicalize().unwrap().display());
    println!("{:?}", file);
    if let Ok(code) = file {
        println!("Parsing! {:?}", code);
        println!("Result: {:?}", parse(code.as_str()));
    } else {
        eprintln!("Could not read file {}", path.display());
    }
}
