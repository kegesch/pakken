use parser::parse;

fn main() {
    let code = "io.test {}";
    println!("Parsing! {:?}", code);
    println!("Result: {:?}", parse(code));
}
