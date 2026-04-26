use crate::compiler::parser::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub fn analyze(ast: &Program) -> Result<(), Error> {
    check_integer_range(ast)?;
    Ok(())
}

fn check_integer_range(ast: &Program) -> Result<(), Error> {
    Ok(())
}
