use crate::parser::ParsedSource;
use color_eyre::{
    Section,
    eyre::{Result, eyre},
};
use tree_sitter::{Query, QueryCursor, StreamingIterator};

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct InvalidIntegerRange(pub super::ErrorMsg);

pub fn analyze(source: &ParsedSource) -> Result<()> {
    let reports = collect_errors(source);

    if !reports.is_empty() {
        let mut error_bytes_buffer = Vec::new();

        let errors = reports.into_iter().fold(eyre!("Noob"), |report, err| {
            err.write(source.source(), &mut error_bytes_buffer).unwrap();

            let report_msg = String::from_utf8(std::mem::take(&mut error_bytes_buffer)).unwrap();

            report.error(InvalidIntegerRange(report_msg))
        });

        return Err(errors);
    }

    Ok(())
}

fn collect_errors<'a>(
    source: &'a ParsedSource,
) -> Vec<ariadne::Report<'a, std::ops::Range<usize>>> {
    let mut reports = Vec::new();

    let mut query_cursor = QueryCursor::new();
    let query = Query::new(&tree_sitter_l1::LANGUAGE.into(), "(intconst) @deine-mudda").unwrap();
    let mut matches = query_cursor.matches(
        &query,
        source.ast().root_node(),
        source.source().text().as_bytes(),
    );

    let source_bytes = source.source().text().as_bytes();
    while let Some(query_match) = matches.next() {
        for capture in query_match.captures {
            let node = capture.node;

            let num_str = node.utf8_text(source_bytes).unwrap();
            let report = source.report(node.byte_range());
            match num_str.parse::<u32>() {
                Ok(num) => {
                    if num > 2u32.pow(31) {
                        let report = report
                            .with_message("Invalid integer")
                            .with_label(
                                ariadne::Label::new(node.byte_range())
                                    .with_message("Integer is too high")
                                    .with_color(ariadne::Color::Red),
                            )
                            .finish();

                        reports.push(report);
                    }
                }

                Err(err) => {
                    let report = source
                        .report(node.byte_range())
                        .with_message(format!("{}", err))
                        .with_label(
                            ariadne::Label::new(node.byte_range())
                                .with_message("Invalid integer")
                                .with_color(ariadne::Color::Red),
                        )
                        .finish();

                    reports.push(report);
                }
            }
        }
    }

    reports
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    #[test]
    fn max_dec_int() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return 2147483648; }");
        super::analyze(&source)
    }

    #[test]
    fn min_dec_int() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return -2147483648; }");
        super::analyze(&source)
    }

    #[test]
    fn too_big_dec_int() {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return 2147483649; }");
        assert!(super::analyze(&source).is_err());
    }

    #[test]
    fn too_small_dec_int() {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return -2147483649; }");
        assert!(super::analyze(&source).is_err());
    }

    // == hex ==

    #[test]
    fn max_hex_int_with_small_x() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return 0x0; }");
        super::analyze(&source)
    }

    #[test]
    fn min_hex_int_with_small_x() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return -0x0; }");
        super::analyze(&source)
    }
    #[test]
    fn max_hex_int_with_big_x() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return 0X0; }");
        super::analyze(&source)
    }

    #[test]
    fn min_hex_int_with_big_x() -> Result<()> {
        crate::init_color_eyre();

        let source = crate::parser::parse("int main() { return -0X0; }");
        super::analyze(&source)
    }
}
