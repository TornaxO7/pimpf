pub fn remove_comments<'src>(code: &'src str) -> Result<String, ()> {
    let mut stripped_code = String::with_capacity(code.len());

    let mut block_comment_level = 0;

    let mut iterator = code.chars().peekable();
    while let Some(current) = iterator.next() {
        // == line comment
        let is_line_comment = {
            let next_is_slash = iterator.peek().map(|c| *c == '/').unwrap_or(false);

            current == '/' && next_is_slash
        };

        if is_line_comment {
            // skip until end of line
            while let Some(c2) = iterator.next() {
                if c2 == '\n' {
                    break;
                }
            }
        }

        // == block comment
        let start_block_comment = {
            let next_is_star = iterator.peek().map(|c| *c == '*').unwrap_or(false);

            current == '/' && next_is_star
        };

        if start_block_comment {
            block_comment_level += 1;
        }

        let end_block_comment = {
            let next_is_slash = iterator.peek().map(|c| *c == '/').unwrap_or(false);

            current == '*' && next_is_slash
        };
        if end_block_comment {
            block_comment_level -= 1;
        }

        // now add stuff or not
        if block_comment_level == 0 {
            stripped_code.push(current);
        }
    }

    if block_comment_level != 0 {
        return Err(());
    }

    Ok(stripped_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comment_simple() {
        assert_eq!(
            remove_comments(
                r#"int main() {
                // I use arch btw.
                }"#
            ),
            Ok(r#"int main() {
                }"#
            .to_string())
        );
    }
}
