use std::collections::HashMap;

use super::Ssa;
use crate::compiler::{
    parser,
    ssa::{BasicBlockId, VariableId, basic_block::BasicBlock},
};

#[derive(Debug)]
pub struct SsaBuilder {
    var_num: u64,
    curr_block: u64,

    vars: HashMap<VariableId, HashMap<BasicBlockId, i32>>,
    ssa: Ssa,
}

impl SsaBuilder {
    // TODO:
    // 1. Split long expresions into simple ones
    // 2. Apply algorithm of paper
    pub fn build(ast: parser::Program) -> Ssa {
        let mut builder = Self {
            var_num: 0,
            curr_block: 0,
            vars: HashMap::new(),
            ssa: Ssa::new(),
        };

        builder.build_block(&ast.0);

        builder.ssa
    }

    fn build_block(&mut self, stmts: &[parser::Statement]) {
        let mut block = BasicBlock::new();

        for stmt in stmts {
            match stmt {
                parser::Statement::Decl(decl) => self.process_declaration(&mut block, decl),
                parser::Statement::Simp(simp) => todo!(),
                parser::Statement::Return(ret) => todo!(),
            }
        }

        todo!()
    }

    fn process_declaration(&mut self, block: &mut BasicBlock, decl: &parser::Declaration) {
        todo!()
    }

    fn process_simp(&mut self, block: &mut BasicBlock, simp: &parser::Simp) {
        todo!()
    }

    fn process_return(&mut self, block: &mut BasicBlock, ret: &parser::Exp) {
        todo!()
    }

    // ===
    fn write_variable(&mut self, var: VariableId, block: BasicBlockId, value: i32) {
        let blocks = self.vars.entry(var).or_insert(HashMap::new());
        blocks.insert(block, value);
    }

    fn read_variable(&self, var: VariableId, block: BasicBlockId) -> Option<i32> {
        let blocks = self.vars.get(&var).expect("Variable is unknown!");

        if let Some(value) = blocks.get(&block) {
            return Some(*value);
        }

        // TODO: Recursive!
        for pre in blocks.predecessors() {
            if let Some(value) = self.read_variable(var, pre) {
                return Some(value);
            }
        }

        todo!()
    }
}
