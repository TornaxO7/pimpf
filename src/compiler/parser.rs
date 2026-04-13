#![allow(dead_code)]
use chumsky::{error::Rich, extra, input::MappedInput, prelude::*};

use super::lexer::Token;

pub type Ident = String;

macro_rules! Parser {
    ($e:ident) => {
        impl Parser<'src, MappedInput<'src, Token, SimpleSpan, &'src [Spanned<Token>]>, $e, extra::Err<Rich<'src, Token>>>
    }
}

#[derive(Debug, Clone)]
pub struct Program(pub Vec<Statement>);

#[derive(Debug, Clone)]
pub enum Statement {
    Decl(Declaration),
    Simp(Simp),
    Return(Exp),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub ty: Type,
    pub ident: Ident,
    pub exp: Option<Exp>,
}

#[derive(Debug, Clone)]
pub struct Simp {
    pub lvalue: Lvalue,
    pub asnop: Asnop,
    pub exp: Exp,
}

#[derive(Debug, Clone)]
pub enum Lvalue {
    Ident(Ident),
    Nested(Box<Lvalue>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Nested(Box<Exp>),
    Intconst(u32),
    Ident(Ident),
    Binop {
        exp1: Box<Self>,
        op: Binop,
        exp2: Box<Self>,
    },
    Unop {
        op: Unop,
        exp: Box<Self>,
    },
}

#[derive(Debug, Clone)]
pub enum Asnop {
    Assign,
    PlusAssign,
    MinusAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binop {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unop {
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
}

pub fn build_ast<'src>(tokens: &'src [Spanned<Token>]) -> ParseResult<Program, Rich<'src, Token>> {
    program_parser().parse(tokens.split_spanned((0..0).into()))
}

fn program_parser<'src>() -> Parser!(Program) {
    type_parser()
        .then_ignore(just(Token::Ident("main".to_string())))
        .then_ignore(just(Token::RoundBracketOpen))
        .then_ignore(just(Token::RoundBracketClose))
        .then_ignore(just(Token::CurlyBracketOpen))
        .then(statement_parser().repeated().collect::<Vec<Statement>>())
        .then_ignore(just(Token::CurlyBracketClose))
        .try_map(|(ty, statements), span| {
            if ty != Type::Int {
                Err(Rich::custom(span, "`main()` function have type `int`"))
            } else {
                Ok(Program(statements))
            }
        })
}

fn type_parser<'src>() -> Parser!(Type) {
    let int = just(Token::Int).to(Type::Int);

    int
}

fn statement_parser<'src>() -> Parser!(Statement) {
    let decl = decl_parser()
        .then_ignore(just(Token::Semicolon))
        .map(|decl| Statement::Decl(decl));

    let simp = simp_parser()
        .then_ignore(just(Token::Semicolon))
        .map(|simp| Statement::Simp(simp));

    let ret = just(Token::Return)
        .ignore_then(exp_parser())
        .then_ignore(just(Token::Semicolon))
        .map(|exp| Statement::Return(exp));

    choice((decl, simp, ret))
}

fn decl_parser<'src>() -> Parser!(Declaration) {
    type_parser()
        .then(select! {Token::Ident(i) => i})
        .then(just(Token::Assign).ignore_then(exp_parser()).or_not())
        .map(|((ty, ident), exp)| Declaration { ty, ident, exp })
}

fn simp_parser<'src>() -> Parser!(Simp) {
    lvalue_parser()
        .then(asnop_parser())
        .then(exp_parser())
        .map(|((lvalue, asnop), exp)| Simp { lvalue, asnop, exp })
}

fn lvalue_parser<'src>() -> Parser!(Lvalue) {
    let nested = recursive(|lvalue| {
        just(Token::RoundBracketOpen)
            .ignore_then(lvalue)
            .then_ignore(just(Token::RoundBracketClose))
            .map(|l| Lvalue::Nested(Box::new(l)))
    });

    let ident = select! {Token::Ident(i) => Lvalue::Ident(i)};

    choice((ident, nested))
}

fn exp_parser<'src>() -> Parser!(Exp) {
    let intconst_ident = {
        let decnum =
            select!(Token::Decnum(int) => int).try_map(|int, span| match int.parse::<u32>() {
                Ok(num) => Ok(Exp::Intconst(num)),
                Err(err) => Err(Rich::custom(span, err.to_string())),
            });

        let hexnum = select!(Token::Hexnum(int) => int).try_map(|int, span| {
            match u32::from_str_radix(&int, 16) {
                Ok(num) => Ok(Exp::Intconst(num)),
                Err(err) => Err(Rich::custom(span, err.to_string())),
            }
        });

        let ident = select!(Token::Ident(ident) => Exp::Ident(ident));

        choice((decnum, hexnum, ident))
    };

    let nested = recursive(|exp| {
        just(Token::RoundBracketOpen)
            .ignore_then(exp)
            .then_ignore(just(Token::RoundBracketClose))
            .map(|e| Exp::Nested(Box::new(e)))
    });

    let unop = recursive(|exp| {
        let unop = select! {
            Token::Minus => Unop::Minus,
        };

        unop.then(exp.clone()).map(|(op, exp1)| Exp::Unop {
            op,
            exp: Box::new(exp1),
        })
    });

    let binop = {
        let exp = choice((intconst_ident, nested.clone(), unop.clone()));

        let binop = select! {
             Token::Plus => Binop::Plus,
             Token::Minus => Binop::Minus,
             Token::Star => Binop::Mult,
             Token::Slash => Binop::Div,
             Token::Percentage => Binop::Mod,
        };

        exp.clone()
            .then(binop)
            .then(exp)
            .map(|((exp1, op), exp2)| Exp::Binop {
                exp1: Box::new(exp1),
                op,
                exp2: Box::new(exp2),
            })
    };

    choice((binop, nested, unop, intconst_ident))
}

fn asnop_parser<'src>() -> Parser!(Asnop) {
    select! {
        Token::Assign => Asnop::Assign,
        Token::PlusAssign => Asnop::PlusAssign,
        Token::MinusAssign => Asnop::MinusAssign,
        Token::StarAssign => Asnop::MulAssign,
        Token::SlashAssign => Asnop::DivAssign,
        Token::PercentageAssign => Asnop::ModAssign,
    }
}

#[cfg(test)]
mod tests {
    use chumsky::span::SpanWrap;

    use super::*;

    #[test]
    fn sandbox() {
        let tokens = [
            Token::Decnum("69".to_string()).with_span(SimpleSpan::from(0..1)),
            Token::Slash.with_span(SimpleSpan::from(1..2)),
            Token::Decnum("0".to_string()).with_span(SimpleSpan::from(2..3)),
        ];

        assert_eq!(
            exp_parser()
                .parse(tokens.as_ref().split_spanned((0..1).into()))
                .unwrap(),
            Exp::Binop {
                exp1: Box::new(Exp::Intconst(69)),
                op: Binop::Div,
                exp2: Box::new(Exp::Intconst(0))
            }
        );
    }
}
