use crate::grammar::*;

pub fn analyze<'src>(p: &Program<'src>) -> Result<(), ()> {
    analyze_statements(&p.0)
}

fn analyze_statements<'src>(statements: &Statements<'src>) -> Result<(), ()> {
    match statements {
        Statements::None => Err(()),
        Statements::Statement { current, next } => {
            match analyze_statement(current) {
                Ok(true) => return Ok(()),
                _ => {}
            };
            analyze_statements(next)
        }
    }
}

fn analyze_statement<'src>(statement: &Statement<'src>) -> Result<bool, ()> {
    match statement {
        Statement::Return(_) => Ok(true),
        _ => Ok(false),
    }
}
