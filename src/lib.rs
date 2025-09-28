mod parser;

use std::path::Path;

use crate::parser::Program;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error(transparent)]
    Parser(#[from] parser::Error),
}

#[derive(Debug, Clone)]
pub struct Compiler {
    source_code: String,
    ast: Option<Program>,
}

impl Compiler {
    pub fn new<S: AsRef<str>>(source_code: S) -> Self {
        Self {
            source_code: source_code.as_ref().to_string(),
            ast: None,
        }
    }

    pub fn compile(&mut self) -> Result<&mut Self, Error> {
        let ast = parser::parse(&self.source_code)?;
        Ok(self)
    }

    pub fn save_binary_to<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        todo!()
    }
}
