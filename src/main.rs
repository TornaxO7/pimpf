use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_l1::LANGUAGE.into())
        .unwrap();

    let source_code = "int main() { return 0;";
    let tree = parser.parse(source_code, None).unwrap();

    println!("{:?}", tree);
}
