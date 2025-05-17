mod syntax;

use color_eyre::eyre::Result;

use crate::parser::ParsedSource;

pub fn analyze(source: &ParsedSource) -> Result<()> {
    syntax::analyze(source)?;
    Ok(())
}
