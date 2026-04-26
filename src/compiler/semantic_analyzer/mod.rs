mod constants_checking;
mod return_checking;

use crate::compiler::parser::Program;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid decnum: {0}")]
    InvalidDecnum(String),

    #[error("Invalid hexnum: {0}")]
    InvalidHexnum(String),

    #[error("Missing return statement")]
    MissingReturn,
}

pub fn analyze(ast: &Program) -> Result<(), Error> {
    return_checking::check(ast)?;
    constants_checking::check(ast)?;

    Ok(())
}
