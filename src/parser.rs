use chumsky::Parser;
use chumsky::prelude::*;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {}

pub fn parse<S: AsRef<str>>(source_code: S) -> Result<Program, Error> {
    todo!()
}

fn decnum_parser<'a>() -> impl Parser<'a, &'a str, Decnum> {
    let zero = just("0").to(Decnum(0));
    let int = one_of('1'..'9')
        .then(one_of('0'..'9').repeated().collect::<String>())
        .map(|(prev, next)| {
            let num_string = {
                let mut s = String::with_capacity(next.len() + 1);
                s.push(prev);
                s.push_str(&next);
                s
            };

            Decnum(num_string.parse::<u32>().unwrap())
        });

    zero.or(int)
}

// == AST ==

#[derive(Debug, Clone)]
pub enum Program {
    Statements(Statements),
}

#[derive(Debug, Clone)]
pub struct Statements(Vec<Statement>);

#[derive(Debug, Clone)]
pub enum Statement {
    Decl(Decl),
    Simpl(Simp),
    Return(Exp),
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub t: Type,
    pub ident: Ident,
    pub exp: Option<Exp>,
}

#[derive(Debug, Clone)]
pub struct Simp {
    lvalue: Lvalue,
    asnop: Asnop,
    exp: Exp,
}

#[derive(Debug, Clone)]
pub enum Lvalue {
    Ident(Ident),
    Nested(Box<Self>),
}

#[derive(Debug, Clone)]
pub enum Exp {
    Nested(Box<Self>),
    IntConst(Intconst),
    Ident(Ident),
    BinExp {
        left: Box<Self>,
        binop: Binop,
        right: Box<Self>,
    },
    UnExp {
        unop: Unop,
        exp: Box<Self>,
    },
}

#[derive(Debug, Clone)]
pub enum Intconst {
    Decnum(Decnum),
    Hexnum(Hexnum),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Decnum(u32);

#[derive(Debug, Clone)]
pub struct Hexnum(u32);

#[derive(Debug, Clone)]
pub enum Unop {
    Neg,
}

#[derive(Debug, Clone)]
pub enum Asnop {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    DivideEqual,
    PercentageEqual,
}

#[derive(Debug, Clone)]
pub enum Binop {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
}

#[derive(Debug, Clone)]
pub struct Ident(String);

#[cfg(test)]
mod tests {
    use super::*;

    mod decnum {
        use super::*;

        #[test]
        fn zero() {
            let parser = decnum_parser();
            assert_eq!(parser.parse("0").unwrap(), Decnum(0));
        }

        #[test]
        fn non_zero() {
            let parser = decnum_parser();
            assert_eq!(parser.parse("1").unwrap(), Decnum(1));
        }

        #[test]
        #[should_panic]
        fn leading_zero() {
            let parser = decnum_parser();
            parser.parse("01").unwrap();
        }

        #[test]
        fn :wq
    }
}
