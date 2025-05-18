use color_eyre::{
    Section,
    eyre::{Result, eyre},
};
use tree_sitter::{Query, QueryCursor, StreamingIterator};

use crate::parser::ParsedSource;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct MissingReturn(super::ErrorMsg);

pub fn analyze(source: &ParsedSource) -> Result<()> {
    let reports = collect_errors(source);
    if !reports.is_empty() {
        let mut error_bytes_buffer = Vec::new();

        let errors = reports
            .into_iter()
            .fold(eyre!("Invalid program"), |report, err| {
                err.write(source.source(), &mut error_bytes_buffer).unwrap();

                let report_msg =
                    String::from_utf8(std::mem::take(&mut error_bytes_buffer)).unwrap();

                report.error(MissingReturn(report_msg))
            });

        return Err(errors);
    }

    Ok(())
}

fn collect_errors<'a>(
    source: &'a ParsedSource,
) -> Vec<ariadne::Report<'a, std::ops::Range<usize>>> {
    let mut has_return_statement = false;
    let source_bytes = source.source().text().as_bytes();

    let query = Query::new(&tree_sitter_l1::LANGUAGE.into(), "(statement) @siu").unwrap();
    let mut query_cursor = QueryCursor::new();
    let mut query_matches = query_cursor.matches(&query, source.ast().root_node(), source_bytes);

    while let Some(query_match) = query_matches.next() {
        for capture in query_match.captures {
            let node = capture.node;

            let statement_str = node.utf8_text(source_bytes).unwrap();
            if statement_str.starts_with("return") {
                has_return_statement = true;
            }
        }
    }

    let mut reports = Vec::new();
    if !has_return_statement {
        let report = source
            .report(0..1)
            .with_message("Missing `return` statement!")
            .finish();

        reports.push(report);
    }
    reports
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    #[test]
    fn has_return() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return 0; }");
        super::analyze(&source)
    }

    #[test]
    fn missing_return() {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { int a = 42; int b = 0xCAFEBABE; }");
        assert!(super::analyze(&source).is_err());
    }
}
