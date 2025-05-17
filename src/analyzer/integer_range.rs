use crate::parser::ParsedSource;
use color_eyre::eyre::Result;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct InvalidIntegerRange(pub super::ErrorMsg);

pub fn analyze(source: &ParsedSource) -> Result<()> {
    todo!()
}
