#![allow(unused_variables)]

mod lexer;
mod parser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error occured during lexing.")]
    Lexer,

    #[error("An error occured during parsing.")]
    Parser,
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Lexer | Self::Parser => 42,
        }
    }
}

pub fn compile<'a>(src: &'a str) -> Result<(), Error> {
    let tokens = lexer::tokenize(src).map_err(|_| Error::Lexer)?;
    let ast = parser::build_ast(tokens).map_err(|_| Error::Parser)?;

    Ok(())
}
