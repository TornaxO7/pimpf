use super::Error;
use crate::compiler::parser::*;

pub fn check(ast: &Program) -> Result<(), Error> {
    for statement in &ast.0 {
        if let Statement::Return(_) = statement {
            return Ok(());
        }
    }

    Err(Error::MissingReturn)
}
