mod builder;

use std::collections::HashMap;

use crate::compiler::ssa::{BasicBlockId, VariableId};

#[derive(Debug)]
pub enum Statement {
    Atom(ExpAtom),

    Add(ExpAtom, ExpAtom),
    Subtract(ExpAtom, ExpAtom),
    Multiply(ExpAtom, ExpAtom),
    Divide(ExpAtom, ExpAtom),
    Mod(ExpAtom, ExpAtom),
}

#[derive(Debug, Clone)]
pub enum ExpAtom {
    Intconst(i32),
    Var(VariableId),
}

#[derive(Debug)]
pub struct BasicBlock {
    predecessor: Vec<BasicBlockId>,
    descendants: Vec<BasicBlockId>,
}

impl BasicBlock {
    pub fn new() -> Self {
        Self {
            predecessor: Vec::new(),
            descendants: Vec::new(),
        }
    }
}
