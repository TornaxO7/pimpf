mod lexer;
mod parser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error occured during lexing.")]
    Lexer,

    #[error("An error occured during parsing.")]
    Parser,
}

pub fn compile<'a>(src: &'a str) -> Result<(), Error> {
    let tokens = lexer::tokenize(src).map_err(|_| Error::Lexer)?;
    let ast = parser::build_ast(tokens).map_err(|_| Error::Parser)?;
    todo!()
}
