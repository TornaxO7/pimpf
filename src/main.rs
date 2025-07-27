mod grammar;
mod parser;

fn main() {
    let mut args = std::env::args();

    let input_file = std::fs::read_to_string(args.nth(1).unwrap()).unwrap();
    // let output_file = args.nth(2).unwrap();

    parser::parse(&input_file).unwrap();
}
