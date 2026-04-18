#![allow(unused_variables)]

use ariadne::Report;

mod lexer;
mod parser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An error occured during lexing.")]
    Lexer,

    #[error("An error occured during parsing.")]
    Parser,
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Lexer | Self::Parser => 42,
        }
    }
}

pub fn compile<'a>(src: &'a str) -> Result<(), Error> {
    let tokens = {
        let tokens = lexer::tokenize(src);

        if tokens.has_errors() {
            for err in tokens.errors() {
                Report::build(ariadne::ReportKind::Error, 0..src.len())
                    .with_label(
                        ariadne::Label::new(err.span().into_range())
                            .with_message(format!("{}", err)),
                    )
                    .with_message(format!("Lexer error"))
                    .finish()
                    .eprint(ariadne::Source::from(src))
                    .unwrap();
            }

            return Err(Error::Lexer);
        }

        tokens.unwrap()
    };

    let ast = {
        let ast = parser::build_ast(&tokens);

        if ast.has_errors() {
            for err in ast.errors() {
                Report::build(ariadne::ReportKind::Error, 0..src.len())
                    .with_label(
                        ariadne::Label::new(err.span().into_range())
                            .with_message(format!("{:?}", err)),
                    )
                    .with_message(format!("Parser error"))
                    .finish()
                    .eprint(ariadne::Source::from(src))
                    .unwrap();
            }

            return Err(Error::Parser);
        }

        ast.unwrap()
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comptest() {
        compile("int main() {return a + b + c;}").unwrap();
    }
}
