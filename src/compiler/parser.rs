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
    Intconst(u32),
    Ident(Ident),

    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
    Mod(Box<Self>, Box<Self>),

    Neg(Box<Self>),
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

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Binop {
//     Plus,
//     Minus,
//     Mult,
//     Div,
//     Mod,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Unop {
//     Minus,
// }

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
    recursive(|lvalue| {
        let nested = just(Token::RoundBracketOpen)
            .ignore_then(lvalue)
            .then_ignore(just(Token::RoundBracketClose))
            .map(|l| Lvalue::Nested(Box::new(l)));

        let ident = select! {Token::Ident(i) => Lvalue::Ident(i)};

        choice((ident, nested))
    })
}

fn exp_parser<'src>() -> Parser!(Exp) {
    recursive(|exp| {
        let intconst = {
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

            choice((decnum, hexnum))
        };

        let ident = select!(Token::Ident(ident) => Exp::Ident(ident));

        let nested = exp.clone().delimited_by(
            just(Token::RoundBracketOpen),
            just(Token::RoundBracketClose),
        );

        let atomic_exp = choice((intconst, ident, nested));

        let op_exp = {
            let op = |c| just(c);

            // unop
            let prec1 = op(Token::Minus)
                .repeated()
                .foldr(atomic_exp, |_op, rhs| Exp::Neg(Box::new(rhs)));

            // '*', '/', '%'
            let prec2 = prec1.clone().foldl(
                choice((
                    op(Token::Star).to(Exp::Multiply as fn(_, _) -> _),
                    op(Token::Slash).to(Exp::Divide as fn(_, _) -> _),
                    op(Token::Percentage).to(Exp::Mod as fn(_, _) -> _),
                ))
                .then(prec1)
                .repeated(),
                |exp1, (op, exp2)| op(Box::new(exp1), Box::new(exp2)),
            );

            // '+', '-'
            let prec3 = prec2.clone().foldl(
                choice((
                    op(Token::Plus).to(Exp::Add as fn(_, _) -> _),
                    op(Token::Minus).to(Exp::Subtract as fn(_, _) -> _),
                ))
                .then(prec2)
                .repeated(),
                |exp1, (op, exp2)| op(Box::new(exp1), Box::new(exp2)),
            );

            prec3
        };

        op_exp
    })
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
    use super::*;

    #[test]
    fn sandbox() {
        let tokens = [
            Spanned {
                inner: Token::Ident("a".to_string()),
                span: SimpleSpan::new((), 0..1),
            },
            Spanned {
                inner: Token::Plus,
                span: SimpleSpan::new((), 1..2),
            },
            Spanned {
                inner: Token::Ident("b".to_string()),
                span: SimpleSpan::new((), 2..3),
            },
            Spanned {
                inner: Token::Plus,
                span: SimpleSpan::new((), 3..4),
            },
            Spanned {
                inner: Token::Ident("c".to_string()),
                span: SimpleSpan::new((), 4..5),
            },
        ];
        let result = exp_parser()
            .parse(tokens.as_ref().split_spanned((0..1).into()))
            .unwrap();
    }
}
