use chumsky::{error::Rich, extra, prelude::*};

use super::lexer::Token;

#[derive(Debug, Clone)]
pub enum Ast {}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Decl {
        ty: Type,
        ident: String,
        exp: Option<Exp>,
    },
}

#[derive(Debug, Clone)]
pub enum Exp {}

pub fn build_ast(tokens: Vec<Token>) -> Result<Ast, ()> {
    Err(())
}
