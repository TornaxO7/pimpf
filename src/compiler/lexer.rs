use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Intconst(u32),
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
    Int,
    Bool,
    Void,
    Char,
    String,
    // Brackets
    LRound,
    RRound,
    LCurly,
    RCurly,
    // other
    Semicolon,
    Assign,
    AssignPlus,
    AssignMinus,
    AssignMul,
    AssignDiv,
    AssignMod,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
}

pub fn tokenize<'a>(src: &'a str) -> Result<Vec<Token>, ()> {
    let brackets = {
        let lround = just('(').to(Token::LRound);
        let rround = just(')').to(Token::RRound);
        let lcurly = just('{').to(Token::LCurly);
        let rcurly = just('}').to(Token::RCurly);

        choice((lround, rround, lcurly, rcurly))
    };

    let intconst = choice((decnum_parser(), hexnum_parser()));

    let asnop = {
        let assign = just('=').to(Token::Assign);
        let assign_plus = just("+=").to(Token::AssignPlus);
        let assign_minus = just("-=").to(Token::AssignMinus);
        let assign_mul = just("*=").to(Token::AssignMul);
        let assign_div = just("/=").to(Token::AssignDiv);
        let assign_mod = just("%=").to(Token::AssignMod);

        choice((
            assign,
            assign_plus,
            assign_minus,
            assign_mul,
            assign_div,
            assign_mod,
        ))
    };

    let binop = {
        let plus = just('+').to(Token::Plus);
        let minus = just('-').to(Token::Minus);
        let mul = just('*').to(Token::Mul);
        let div = just('/').to(Token::Div);
        let r#mod = just('%').to(Token::Mod);

        choice((plus, minus, mul, div, r#mod))
    };

    let semicolon = just(';').to(Token::Semicolon);

    let token_parser = choice((ident_parser(), brackets, intconst, asnop, binop, semicolon));

    let whitespace = one_of("\t\r\n ").ignored();
    let comment = {
        let newline = one_of("\r\n");

        let line_comment = just("//")
            .ignore_then(any().and_is(newline).not().repeated())
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

    let result = token_parser
        .padded_by(choice((comment, whitespace)))
        .repeated()
        .collect::<Vec<Token>>()
        .parse(src);

    if result.has_errors() {
        for err in result.into_errors() {
            eprintln!("{}", err);
        }

        return Err(());
    }

    Ok(result.unwrap())
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
    let just0 = just('0').to(Token::Intconst(0));
    let non_zero = one_of('1'..='9')
        .then(one_of('0'..='9').repeated().collect::<String>())
        .map(|(p, s)| format!("{}{}", p, s))
        .map(|num| Token::Intconst(num.parse().unwrap()));

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
        .map(|hex| u32::from_str_radix(&hex, 16).unwrap())
        .map(|value| Token::Intconst(value))
}
