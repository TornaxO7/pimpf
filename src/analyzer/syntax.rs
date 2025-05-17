use color_eyre::{
    Section,
    eyre::{Result, eyre},
};

use crate::parser::ParsedSource;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct SyntaxError(pub String); // contains message

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
    let mut cursor = source.ast().walk();
    let mut reports = Vec::new();

    'tree_traversal: loop {
        let node = cursor.node();
        if node.is_error() {
            let report = source
                .report(ariadne::ReportKind::Error, node.byte_range())
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
                .report(ariadne::ReportKind::Error, node.byte_range())
                .with_label(
                    ariadne::Label::new(node.byte_range())
                        .with_message(format!("'{}' is missing", node.kind()))
                        .with_color(ariadne::Color::Red),
                )
                .finish();

            reports.push(report);
        }

        // move to next node

        if cursor.goto_first_child() {
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        'up_traversal: loop {
            if cursor.goto_parent() {
                if cursor.goto_next_sibling() {
                    break 'up_traversal;
                }
            } else {
                // we are back at the root
                break 'tree_traversal;
            }
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
    fn missing_equal_sign() -> Result<()> {
        crate::init_color_eyre();

        let source = "int main() { int a 10; return 0; }";
        let ast = crate::parser::parse(source);

        // assert!(super::analyze(&ast).is_err());
        super::analyze(&ast)
    }

    #[test]
    fn missing_bracket() {
        crate::init_color_eyre();

        let source = "int main() { return 0; ";
        let ast = crate::parser::parse(source);

        assert!(super::analyze(&ast).is_err());
    }
}
