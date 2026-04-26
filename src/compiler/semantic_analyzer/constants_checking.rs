use super::Error;
use crate::compiler::parser::*;

pub fn check(ast: &Program) -> Result<(), Error> {
    let is_valid = |exp: &Exp| match exp {
        Exp::Decnum(decnum) => {
            if decnum.parse::<u32>().is_ok() {
                Ok(())
            } else {
                Err(Error::InvalidDecnum(decnum.clone()))
            }
        }
        Exp::Hexnum(hexnum) => {
            if u32::from_str_radix(hexnum.as_str(), 16).is_ok() {
                Ok(())
            } else {
                Err(Error::InvalidHexnum(hexnum.clone()))
            }
        }
        _ => Ok(()),
    };

    for statement in &ast.0 {
        match statement {
            Statement::Decl(decl) => {
                if let Some(exp) = &decl.exp {
                    is_valid(exp)?;
                }
            }
            Statement::Simp(simp) => is_valid(&simp.exp)?,
            Statement::Return(exp) => is_valid(exp)?,
        }
    }
    Ok(())
}
