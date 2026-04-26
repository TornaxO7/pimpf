mod integer_range;
mod main_has_return;
mod variables;

use crate::compiler::parser::Program;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid intconst: {0}")]
    InvalidIntconst(String),

    #[error("Missing return statement")]
    MissingReturn,

    #[error(transparent)]
    Variable(variables::Error),
}

pub fn analyze(ast: &Program) -> Result<(), Error> {
    variables::check(ast).map_err(Error::Variable)?;
    integer_range::check(ast)?;
    main_has_return::check(ast)?;

    Ok(())
}
