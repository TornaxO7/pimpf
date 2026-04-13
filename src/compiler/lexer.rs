use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Decnum(String),
    Hexnum(String),
    // Reserved keywords
    Struct,
    If,
    Else,
    While,
    For,
    Continue,
    Break,
    Return,
    Assert,
    True,
    False,
    Null,
    Print,
    Read,
    Alloc,
    AllocArray,
    // types
    Int,
    Bool,
    Void,
    Char,
    String,
    // Brackets
    RoundBracketOpen,
    RoundBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    // other
    Semicolon,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentageAssign,
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
}

pub fn tokenize<'a>(src: &'a str) -> ParseResult<Vec<Spanned<Token>>, Rich<'a, char>> {
    let brackets = {
        let round_bracket_open = just('(').to(Token::RoundBracketOpen).spanned();
        let round_bracket_close = just(')').to(Token::RoundBracketClose).spanned();
        let curly_bracket_open = just('{').to(Token::CurlyBracketOpen).spanned();
        let curly_bracket_close = just('}').to(Token::CurlyBracketClose).spanned();

        choice((
            round_bracket_open,
            round_bracket_close,
            curly_bracket_open,
            curly_bracket_close,
        ))
    };

    let intconst = choice((hexnum_parser().spanned(), decnum_parser().spanned()));

    let asnop = {
        let assign = just('=').to(Token::Assign).spanned();
        let plus_assign = just("+=").to(Token::PlusAssign).spanned();
        let minus_assign = just("-=").to(Token::MinusAssign).spanned();
        let star_assign = just("*=").to(Token::StarAssign).spanned();
        let slash_assign = just("/=").to(Token::SlashAssign).spanned();
        let percentage_assign = just("%=").to(Token::PercentageAssign).spanned();

        choice((
            assign,
            plus_assign,
            minus_assign,
            star_assign,
            slash_assign,
            percentage_assign,
        ))
    };

    let binop = {
        let plus = just('+').to(Token::Plus).spanned();
        let minus = just('-').to(Token::Minus).spanned();
        let star = just('*').to(Token::Star).spanned();
        let slash = just('/').to(Token::Slash).spanned();
        let percentage = just('%').to(Token::Percentage).spanned();

        choice((plus, minus, star, slash, percentage))
    };

    let semicolon = just(';').to(Token::Semicolon).spanned();

    let lexer = choice((
        ident_parser().spanned(),
        brackets,
        intconst,
        asnop,
        binop,
        semicolon,
    ));

    let whitespace = one_of("\t\r\n ").ignored();
    let comment = {
        let newline = one_of("\r\n");

        let line_comment = just("//")
            .ignore_then(any().and_is(newline.not()).repeated())
            .ignored();

        let block_comment = recursive(|comment| {
            let suffix = just("*/");
            let prefix = just("/*");
            let body = any().and_is(suffix.not()).repeated();

            prefix
                .ignore_then(comment.or(body).repeated().ignored())
                .ignore_then(suffix)
                .ignored()
        });

        line_comment.or(block_comment)
    };

    lexer
        .padded_by(choice((comment, whitespace)).repeated().or_not())
        .repeated()
        .collect::<Vec<Spanned<Token>>>()
        .parse(src)
}

fn ident_parser<'src>() -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char>>> {
    let prefix = choice((one_of('A'..='Z'), one_of('a'..='z'), just('_')));
    let suffix = choice((
        one_of('A'..='Z'),
        one_of('a'..='z'),
        one_of('0'..='9'),
        just('_'),
    ))
    .repeated()
    .collect::<String>();

    let parser = prefix.then(suffix).map(|(p, s)| format!("{}{}", p, s));

    parser.map(|ident| match ident.as_str() {
        "struct" => Token::Struct,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "for" => Token::For,
        "continue" => Token::Continue,
        "break" => Token::Break,
        "return" => Token::Return,
        "assert" => Token::Assert,
        "true" => Token::True,
        "false" => Token::False,
        "NULL" => Token::Null,
        "print" => Token::Print,
        "read" => Token::Read,
        "alloc" => Token::Alloc,
        "alloc_array" => Token::AllocArray,
        "int" => Token::Int,
        "bool" => Token::Bool,
        "void" => Token::Void,
        "char" => Token::Char,
        "string" => Token::String,
        other => Token::Ident(other.to_string()),
    })
}

fn decnum_parser<'src>() -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char>>> {
    let just0 = just('0').to(Token::Decnum("0".to_string()));
    let non_zero = one_of('1'..='9')
        .then(one_of('0'..='9').repeated().collect::<String>())
        .map(|(p, s)| format!("{}{}", p, s))
        .map(|d| Token::Decnum(d));

    just0.or(non_zero)
}

fn hexnum_parser<'src>() -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char>>> {
    just('0')
        .ignore_then(one_of("xX"))
        .ignore_then(
            choice((one_of('A'..='F'), one_of('a'..='f'), one_of('0'..='9')))
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .map(|hex| Token::Hexnum(hex))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox() {
        let token = tokenize("0xfffffffff").unwrap();

        assert_eq!(token[0].inner, Token::Hexnum("fffffffff".to_string()))
    }

    mod ident {
        use super::*;

        #[test]
        fn int_type() {
            assert_eq!(ident_parser().parse("int").unwrap(), Token::Int);
        }
    }
}
