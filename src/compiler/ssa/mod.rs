mod basic_block;
mod builder;

use crate::compiler::ssa::basic_block::BasicBlock;
use std::collections::HashMap;

type BasicBlockId = usize;
type VariableId = usize;

#[derive(Debug)]
pub struct Ssa {
    basic_blocks: HashMap<BasicBlockId, BasicBlock>,
}

impl Ssa {
    pub fn new() -> Self {
        Self {
            basic_blocks: HashMap::new(),
        }
    }
}

pub fn build(ast: crate::compiler::parser::Program) -> Ssa {
    builder::SsaBuilder::build(ast)
}
