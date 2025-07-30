use crate::grammar::*;

pub fn analyze<'src>(p: &Program<'src>) -> Result<(), ()> {
    analyze_statements(&p.0)
}

fn analyze_statements<'src>(statements: &Statements<'src>) -> Result<(), ()> {
    match statements {
        Statements::None => Ok(()),
        Statements::Statement { current, next } => {
            analyze_statement(current)?;
            analyze_statements(next)
        }
    }
}

fn analyze_statement<'src>(statement: &Statement<'src>) -> Result<(), ()> {
    match statement {
        Statement::Decl(decl) => analyze_decl(decl),
        Statement::Simp(simp) => analyze_simp(simp),
        Statement::Return(exp) => analyze_exp(exp),
    }
}

fn analyze_decl<'src>(decl: &Declaration<'src>) -> Result<(), ()> {
    match decl {
        Declaration::Ident(ident) => analyze_ident(ident),
        Declaration::IdentExp { ident, exp } => {
            analyze_ident(ident)?;
            analyze_exp(exp)
        }
    }
}

fn analyze_simp<'src>(simp: &SimpleInstruction<'src>) -> Result<(), ()> {
    let SimpleInstruction { lvalue, exp, .. } = simp;

    analyze_lvalue(lvalue)?;
    analyze_exp(exp)?;

    Ok(())
}

fn analyze_exp<'src>(exp: &Expression<'src>) -> Result<(), ()> {
    match exp {
        Expression::NestedExp(nested_exp) => analyze_exp(nested_exp),
        Expression::Intconst(intconst) => analyze_intconst(intconst),
        Expression::Ident(ident) => analyze_ident(ident),
        Expression::Binop { left, right, .. } => {
            analyze_exp(left)?;
            analyze_exp(right)?;
            Ok(())
        }
        Expression::Unop { right, .. } => analyze_exp(right),
    }
}

fn analyze_ident<'src>(_ident: &Identifier<'src>) -> Result<(), ()> {
    Ok(())
}

fn analyze_lvalue<'src>(lvalue: &LValue<'src>) -> Result<(), ()> {
    match lvalue {
        LValue::Ident(ident) => analyze_ident(ident),
        LValue::LValue(lvalue) => analyze_lvalue(lvalue),
    }
}

fn analyze_intconst<'src>(intconst: &Intconst<'src>) -> Result<(), ()> {
    match intconst {
        Intconst::Decnum(decnum) => analyze_decnum(decnum),
        Intconst::Hexnum(hexnum) => analyze_hexnum(hexnum),
    }
}

fn analyze_decnum<'src>(decnum: &Decnum<'src>) -> Result<(), ()> {
    match decnum.0.parse::<u32>() {
        Ok(num) => {
            if num > 1u32 << 31 {
                return Err(());
            }

            Ok(())
        }
        Err(_) => Err(()),
    }
}

fn analyze_hexnum<'src>(hexnum: &Hexnum<'src>) -> Result<(), ()> {
    match u32::from_str_radix(hexnum.0, 16) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // == analyze_decnum
    #[test]
    fn analyze_decnum_min() {
        assert!(analyze_decnum(&Decnum("0")).is_ok());
    }

    #[test]
    fn analyze_decnum_max() {
        assert!(analyze_decnum(&Decnum(&format!("{}", 1u32 << 31))).is_ok());
    }

    #[test]
    fn analyze_decnum_max_exceed() {
        assert!(analyze_decnum(&Decnum(&format!("{}", (1u32 << 31) + 1))).is_err());
    }

    // == analyze_hexnum
    #[test]
    fn analyze_hexnum_min() {
        assert!(analyze_hexnum(&Hexnum("0")).is_ok());
    }

    #[test]
    fn analyze_hexnum_max() {
        assert!(analyze_hexnum(&Hexnum("ffffffff")).is_ok());
    }

    #[test]
    fn analyze_hexnum_max_exceed() {
        assert!(analyze_hexnum(&Hexnum("ffffffff0")).is_err());
    }
}
