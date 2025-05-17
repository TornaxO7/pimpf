mod integer_range;
mod syntax;

use crate::parser::ParsedSource;
use color_eyre::eyre::Result;

type ErrorMsg = String;

pub fn analyze(source: &ParsedSource) -> Result<()> {
    syntax::analyze(source)?;
    integer_range::analyze(source)?;
    Ok(())
}
