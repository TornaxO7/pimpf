use color_eyre::eyre::Result;
use std::path::Path;
use tree_sitter::{Parser, Tree};

#[derive(Debug)]
pub struct ParsedSource {
    content: ariadne::Source<String>,
    ast: Tree,
}

impl ParsedSource {
    pub fn report<'a>(
        &'a self,
        kind: ariadne::ReportKind<'a>,
        range: std::ops::Range<usize>,
    ) -> ariadne::ReportBuilder<'a, std::ops::Range<usize>> {
        ariadne::Report::build(kind, range)
    }

    pub fn source(&self) -> &ariadne::Source {
        &self.content
    }

    pub fn ast(&self) -> &Tree {
        &self.ast
    }
}

pub fn from_path<P: AsRef<Path>>(source: P) -> Result<ParsedSource> {
    let path = source.as_ref();
    let source_content = std::fs::read_to_string(path)?;

    Ok(parse(source_content))
}

pub fn parse<S: AsRef<str>>(content: S) -> ParsedSource {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_l1::LANGUAGE.into())
        .unwrap();

    let ast = parser.parse(content.as_ref(), None).unwrap();

    ParsedSource {
        content: ariadne::Source::from(content.as_ref().to_string()),
        ast,
    }
}
