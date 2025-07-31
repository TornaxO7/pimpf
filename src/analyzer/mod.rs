mod num_range;
mod variable_dec_init_and_usage;

use crate::grammar::Program;

pub fn analyze<'src>(program: &Program<'src>) -> Result<(), ()> {
    num_range::analyze(program)?;
    variable_dec_init_and_usage::analyze(program)?;

    Ok(())
}
