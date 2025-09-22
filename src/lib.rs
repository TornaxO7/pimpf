use std::path::Path;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {}

#[derive(Debug, Clone)]
pub struct Compiler {}

impl Compiler {
    pub fn new<S: AsRef<str>>(source_code: S) -> Self {
        todo!()
    }

    pub fn compile(&mut self) -> Result<&mut Self, Error> {
        Ok(self)
    }

    pub fn save_binary_to<P: AsRef<Path>>(&self, path: P) -> Result<(), std::io::Error> {
        todo!()
    }
}
