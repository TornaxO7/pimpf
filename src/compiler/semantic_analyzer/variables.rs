use crate::compiler::parser::*;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),

    #[error("{0} isn't declared but tried to be read from.")]
    NotDeclared(String),

    #[error("{0} isn't initialised but needed to be.")]
    NotInitialised(String),

    #[error("{0} is already declared.")]
    AlreadyDeclared(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Declared,
    Initialised,
}

struct Analyzer<'a> {
    vars: HashMap<&'a str, State>,
}

impl<'a> Analyzer<'a> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn process_decl(&mut self, decl: &'a Declaration) -> Result<(), Error> {
        let already_declared = self.vars.contains_key(decl.ident.as_str());
        if already_declared {
            return Err(Error::AlreadyDeclared(decl.ident.clone()));
        }

        let only_decl = decl.exp.is_none();
        let state = if let Some(exp) = &decl.exp {
            self.process_exp(exp)?;
            State::Initialised
        } else {
            State::Declared
        };

        self.vars.insert(decl.ident.as_str(), state);
        Ok(())
    }

    pub fn process_simp(&mut self, simp: &'a Simp) -> Result<(), Error> {
        let Some(state) = self.vars.get(simp.lvalue.as_str()) else {
            return Err(Error::NotDeclared(simp.lvalue.clone()));
        };

        match simp.asnop {
            Asnop::Assign => {}
            Asnop::PlusAssign
            | Asnop::MinusAssign
            | Asnop::MulAssign
            | Asnop::DivAssign
            | Asnop::ModAssign => {
                if *state != State::Initialised {
                    return Err(Error::NotInitialised(simp.lvalue.clone()));
                }
            }
        };

        self.vars.insert(simp.lvalue.as_str(), State::Initialised);
        self.process_exp(&simp.exp)
    }

    pub fn process_exp(&mut self, exp: &Exp) -> Result<(), Error> {
        match exp {
            Exp::Decnum(_) | Exp::Hexnum(_) => Ok(()),
            Exp::Ident(ident) => {
                if let Some(state) = self.vars.get(ident.as_str()) {
                    match state {
                        State::Declared => return Err(Error::NotInitialised(ident.clone())),
                        State::Initialised => Ok(()),
                    }
                } else {
                    return Err(Error::UnknownVariable(ident.clone()));
                }
            }
            Exp::Add(a, b)
            | Exp::Subtract(a, b)
            | Exp::Multiply(a, b)
            | Exp::Divide(a, b)
            | Exp::Mod(a, b) => {
                self.process_exp(a)?;
                self.process_exp(b)
            }

            Exp::Neg(a) => self.process_exp(a),
        }
    }
}

pub fn check(ast: &Program) -> Result<(), Error> {
    let mut analyzer = Analyzer::new();

    for statement in &ast.0 {
        match statement {
            Statement::Decl(decl) => analyzer.process_decl(decl)?,
            Statement::Simp(simp) => analyzer.process_simp(simp)?,
            Statement::Return(exp) => analyzer.process_exp(exp)?,
        }
    }

    Ok(())
}
