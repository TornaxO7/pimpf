mod integer_range;
mod main_has_return;

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
    integer_range::check(ast)?;
    main_has_return::check(ast)?;

    Ok(())
}
