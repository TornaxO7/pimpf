use super::Error;
use crate::compiler::parser::*;

pub fn check(ast: &Program) -> Result<(), Error> {
    for statement in &ast.0 {
        match statement {
            Statement::Decl(decl) => {
                if let Some(exp) = &decl.exp {
                    check_exp(exp)?;
                }
            }
            Statement::Simp(simp) => check_exp(&simp.exp)?,
            Statement::Return(exp) => check_exp(exp)?,
        }
    }
    Ok(())
}

fn check_exp(exp: &Exp) -> Result<(), Error> {
    match exp {
        Exp::Ident(_) => Ok(()),
        Exp::Add(a, b)
        | Exp::Subtract(a, b)
        | Exp::Multiply(a, b)
        | Exp::Divide(a, b)
        | Exp::Mod(a, b) => {
            check_exp(a)?;
            check_exp(b)
        }
        Exp::Neg(a) => check_exp(a),
        Exp::Decnum(decnum) => {
            if decnum.parse::<i32>().is_ok() {
                Ok(())
            } else {
                Err(Error::InvalidIntconst(decnum.clone()))
            }
        }
        Exp::Hexnum(hexnum) => {
            if u32::from_str_radix(hexnum.as_str(), 16).is_ok() {
                Ok(())
            } else {
                Err(Error::InvalidIntconst(format!("0x{}", hexnum.clone())))
            }
        }
    }
}
