mod num_range;

use crate::grammar::Program;

pub fn analyze<'src>(program: &Program<'src>) -> Result<(), ()> {
    num_range::analyze(program)?;

    Ok(())
}
