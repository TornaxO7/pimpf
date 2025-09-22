use pimpf::Compiler;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum ArgIndex {
    InputFile = 1,
    OutputFile = 2,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("The input file is missing.")]
    MissingInputFile,

    #[error("The output file is missing.")]
    MissingOutputFile,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    CompileError(#[from] pimpf::Error),
}

fn main() -> Result<(), Error> {
    let mut args = std::env::args();

    let input_file_path = args
        .nth(ArgIndex::InputFile as usize)
        .map(|path| PathBuf::from(path))
        .ok_or(Error::MissingInputFile)?;

    let output_file_path = args
        .nth(ArgIndex::OutputFile as usize)
        .map(|path| PathBuf::from(path))
        .ok_or(Error::MissingOutputFile)?;

    let input_file_content = std::fs::read_to_string(input_file_path)?;

    Compiler::new(input_file_content)
        .compile()?
        .save_binary_to(output_file_path)?;

    Ok(())
}
