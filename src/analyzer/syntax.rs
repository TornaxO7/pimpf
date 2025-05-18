use color_eyre::{
    Section,
    eyre::{Result, eyre},
};

use crate::parser::ParsedSource;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct SyntaxError(pub super::ErrorMsg);

pub fn analyze(source: &ParsedSource) -> Result<()> {
    let node = source.ast().root_node();
    if node.has_error() {
        let mut error_bytes_buffer = Vec::new();

        let errors =
            collect_errors(source)
                .into_iter()
                .fold(eyre!("Skill issue"), |report, err| {
                    err.write(source.source(), &mut error_bytes_buffer).unwrap();

                    let report_msg =
                        String::from_utf8(std::mem::take(&mut error_bytes_buffer)).unwrap();

                    report.error(SyntaxError(report_msg))
                });

        return Err(errors);
    }

    Ok(())
}

fn collect_errors<'a>(
    source: &'a ParsedSource,
) -> Vec<ariadne::Report<'a, std::ops::Range<usize>>> {
    // TODO: Probier das mal mit queries aus
    let mut reports = Vec::new();
    let traverser =
        tree_sitter_traversal::traverse_tree(source.ast(), tree_sitter_traversal::Order::Pre);

    for node in traverser {
        if node.is_error() {
            let report = source
                .report(node.byte_range())
                .with_message("Syntax error")
                .with_label(
                    ariadne::Label::new(node.byte_range())
                        .with_message("sus")
                        .with_color(ariadne::Color::Red),
                )
                .finish();

            reports.push(report);
        } else if node.is_missing() {
            let report = source
                .report(node.byte_range())
                .with_label(
                    ariadne::Label::new(node.byte_range())
                        .with_message(format!("'{}' is missing", node.kind()))
                        .with_color(ariadne::Color::Red),
                )
                .finish();

            reports.push(report);
        }
    }

    reports
}

#[cfg(test)]
mod tests {

    use color_eyre::eyre::Result;

    #[test]
    fn valid_code() -> Result<()> {
        crate::init_color_eyre();

        let source = "int main() { return 0; }";
        let ast = crate::parser::parse(source);

        super::analyze(&ast)
    }

    #[test]
    fn missing_equal_sign() {
        crate::init_color_eyre();

        let source = "int main() { int a 10; return 0; }";
        let ast = crate::parser::parse(source);

        assert!(super::analyze(&ast).is_err());
    }

    #[test]
    fn missing_bracket() {
        crate::init_color_eyre();

        let source = "int main() { return 0; ";
        let ast = crate::parser::parse(source);

        assert!(super::analyze(&ast).is_err());
    }
}
