mod integer_range;
use crate::compiler::parser::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid decnum: {0}")]
    InvalidDecnum(String),

    #[error("Invalid hexnum: {0}")]
    InvalidHexnum(String),
}

pub fn analyze(ast: &Program) -> Result<(), Error> {
    integer_range::check(ast)?;

    Ok(())
}
