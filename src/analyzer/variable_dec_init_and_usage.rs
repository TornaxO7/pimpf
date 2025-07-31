use std::collections::HashSet;

use crate::grammar::*;

pub fn analyze<'src>(p: &Program<'src>) -> Result<(), ()> {
    let mut ana = Analyzer::new();
    ana.analyze(p)
}

#[derive(Debug, Default)]
struct Analyzer<'src> {
    declared: HashSet<&'src str>,
    initialised: HashSet<&'src str>,
}

impl<'src> Analyzer<'src> {
    fn new() -> Self {
        Self::default()
    }

    fn analyze(&mut self, p: &Program<'src>) -> Result<(), ()> {
        self.analyze_statements(&p.0)
    }

    fn analyze_statements(&mut self, statements: &Statements<'src>) -> Result<(), ()> {
        match statements {
            Statements::None => Ok(()),
            Statements::Statement { current, next } => {
                self.analyze_statement(current)?;
                self.analyze_statements(next)
            }
        }
    }

    fn analyze_statement(&mut self, statement: &Statement<'src>) -> Result<(), ()> {
        match statement {
            Statement::Decl(decl) => self.analyze_decl(decl),
            Statement::Simp(simp) => self.analyze_simp(simp),
            Statement::Return(exp) => self.analyze_exp(exp),
        }
    }

    fn analyze_decl(&mut self, decl: &Declaration<'src>) -> Result<(), ()> {
        match decl {
            Declaration::Ident(ident) => {
                if self.declared.contains(ident.0) {
                    return Err(());
                }

                self.declared.insert(ident.0);
                Ok(())
            }
            Declaration::IdentExp { ident, exp } => {
                if self.declared.contains(ident.0) {
                    return Err(());
                }

                self.declared.insert(ident.0);
                self.initialised.insert(ident.0);
                self.analyze_exp(exp)
            }
        }
    }

    fn analyze_simp(&mut self, simp: &SimpleInstruction<'src>) -> Result<(), ()> {
        let SimpleInstruction { lvalue, exp, .. } = simp;

        self.analyze_lvalue(lvalue)?;
        self.analyze_exp(exp)?;

        Ok(())
    }

    fn analyze_exp(&mut self, exp: &Expression<'src>) -> Result<(), ()> {
        match exp {
            Expression::NestedExp(nested_exp) => self.analyze_exp(nested_exp),
            Expression::Ident(ident) => {
                if !self.initialised.contains(ident.0) {
                    return Err(());
                }

                Ok(())
            }
            Expression::Binop { left, right, .. } => {
                self.analyze_exp(left)?;
                self.analyze_exp(right)?;
                Ok(())
            }
            Expression::Unop { right, .. } => self.analyze_exp(right),
            _ => Ok(()),
        }
    }

    fn analyze_lvalue(&mut self, lvalue: &LValue<'src>) -> Result<(), ()> {
        match lvalue {
            LValue::Ident(ident) => {
                if !self.declared.contains(ident.0) {
                    return Err(());
                }
                Ok(())
            }
            LValue::LValue(lvalue) => self.analyze_lvalue(lvalue),
        }
    }
}
