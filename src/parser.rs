use crate::grammar::*;
use chumsky::{Parser, prelude::*};

type ParseError<'src> = extra::Err<Rich<'src, char>>;

pub fn parse<'src>(code: &'src str) -> ParseResult<Program<'src>, Rich<'src, char>> {
    parser().parse(code)
}

fn parser<'src>() -> impl Parser<'src, &'src str, Program<'src>, ParseError<'src>> {
    just("int")
        .ignored()
        .padded()
        .then_ignore(just("main").padded())
        .then_ignore(just("()").padded())
        .then_ignore(just("{").padded())
        .then(statements_parser())
        .then_ignore(just("}").padded())
        .map(|(_main_type, statements)| Program(statements))
}

fn statements_parser<'src>() -> impl Parser<'src, &'src str, Statements<'src>, ParseError<'src>> {
    let none = empty().padded().to(Statements::None);
    let statements = recursive(|statements_parser| {
        statement_parser()
            .boxed()
            .then(statements_parser.or(none.clone()))
            .map(|(statement, next)| Statements::Statement {
                current: statement,
                next: Box::new(next),
            })
    });

    choice((statements, none))
}

fn statement_parser<'src>() -> impl Parser<'src, &'src str, Statement<'src>, ParseError<'src>> {
    let decl = decl_parser()
        .then_ignore(just(';'))
        .map(|decl| Statement::Decl(decl));
    let simp = simp_parser()
        .then_ignore(just(';'))
        .map(|simp| Statement::Simp(simp));
    let ret = just("return")
        .ignored()
        .padded()
        .then(exp_parser())
        .then_ignore(just(';').padded())
        .map(|(_, exp)| Statement::Return(exp));

    choice((decl, simp, ret)).padded()
}

fn decl_parser<'src>() -> impl Parser<'src, &'src str, Declaration<'src>, ParseError<'src>> {
    let init = just("int")
        .ignored()
        .padded()
        .then(ident_parser())
        .then_ignore(just("=").padded())
        .then(exp_parser())
        .map(|((_, ident), exp)| Declaration::IdentExp { ident, exp });

    let decl = just("int")
        .ignored()
        .padded()
        .then(ident_parser())
        .map(|(_, ident)| Declaration::Ident(ident));

    choice((init, decl)).padded()
}

fn simp_parser<'src>() -> impl Parser<'src, &'src str, SimpleInstruction<'src>, ParseError<'src>> {
    lvalue_parser()
        .then(asnop_parser())
        .then(exp_parser())
        .map(|((lvalue, asnop), exp)| SimpleInstruction { lvalue, asnop, exp })
}

fn lvalue_parser<'src>() -> impl Parser<'src, &'src str, LValue<'src>, ParseError<'src>> {
    let ident = ident_parser().padded().map(|ident| LValue::Ident(ident));
    let lvalue = recursive(|lvalue_parser| {
        just('(')
            .ignored()
            .padded()
            .then(lvalue_parser)
            .then_ignore(just(')').padded())
            .map(|(_, lvalue)| LValue::LValue(Box::new(lvalue)))
    });

    choice((ident, lvalue)).padded()
}

fn exp_parser<'src>() -> impl Parser<'src, &'src str, Expression<'src>, ParseError<'src>> {
    let nested_exp = recursive(|exp_parser| {
        just('(')
            .ignored()
            .padded()
            .then(exp_parser)
            .then_ignore(just(')').padded())
            .map(|(_, e)| Expression::Exp(Box::new(e)))
    });

    let intconst = intconst_parser().padded().map(|i| Expression::Intconst(i));
    let ident = ident_parser()
        .padded()
        .map(|ident| Expression::Ident(ident));

    let binop_exp = recursive(|exp_parser| {
        exp_parser
            .clone()
            .then(binop_parser().boxed())
            .then(exp_parser.clone())
            .map(|((left, binop), right)| Expression::Binop {
                left: Box::new(left),
                op: binop,
                right: Box::new(right),
            })
    });

    let unop_exp = recursive(|exp_parser| {
        unop_parser()
            .boxed()
            .then(exp_parser)
            .map(|(unop, exp)| Expression::Unop {
                op: unop,
                right: Box::new(exp),
            })
    });

    choice((nested_exp, intconst, ident, binop_exp, unop_exp)).padded()
}

fn intconst_parser<'src>() -> impl Parser<'src, &'src str, Intconst<'src>, ParseError<'src>> {
    let dec = decnum_parser().map(|d| Intconst::Decnum(d));
    let hex = hexnum_parser().map(|h| Intconst::Hexnum(h));

    choice((dec, hex)).padded()
}

fn unop_parser<'src>() -> impl Parser<'src, &'src str, UnOperation, ParseError<'src>> {
    just('-').to(UnOperation::Minus).padded()
}

fn asnop_parser<'src>() -> impl Parser<'src, &'src str, AsNop, ParseError<'src>> {
    let equal = just("=").to(AsNop::Equal);
    let plus = just("+=").to(AsNop::PlusEqual);
    let minus = just("-=").to(AsNop::MinusEqual);
    let mul = just("*=").to(AsNop::MultEqual);
    let div = just("/=").to(AsNop::DivEqual);
    let r#mod = just("%=").to(AsNop::ModEqual);

    choice((equal, plus, minus, mul, div, r#mod)).padded()
}

fn binop_parser<'src>() -> impl Parser<'src, &'src str, BinOperation, ParseError<'src>> {
    one_of("+-*/%")
        .map(|op| match op {
            '+' => BinOperation::Plus,
            '-' => BinOperation::Minus,
            '*' => BinOperation::Multiplication,
            '/' => BinOperation::Division,
            '%' => BinOperation::Mod,
            _ => unreachable!("Wallah Krise"),
        })
        .padded()
}

fn ident_parser<'src>() -> impl Parser<'src, &'src str, Identifier<'src>, ParseError<'src>> {
    #[rustfmt::skip]
    let prefix = {
        choice( (
            one_of('A'..='Z'),
            one_of('a'..='z'),
            just('_')
        ))
        };

    let suffix = choice((
        one_of('A'..='Z'),
        one_of('a'..='z'),
        one_of('0'..='9'),
        just('_'),
    ))
    .repeated();

    prefix
        .then(suffix)
        .to_slice()
        .padded()
        .map(|ident| Identifier(ident))
}

fn decnum_parser<'src>() -> impl Parser<'src, &'src str, Decnum<'src>, ParseError<'src>> {
    let decnum = one_of('1'..'9')
        .then(one_of('0'..='9').repeated())
        .to_slice()
        .map(|dec| Decnum(dec));
    let just_zero = just("0").map(|z| Decnum(z));

    choice((decnum, just_zero)).padded()
}

#[rustfmt::skip]
fn hexnum_parser<'src>() -> impl Parser<'src, &'src str, Hexnum<'src>, ParseError<'src>> {
    just('0')
        .then(one_of("xX"))
        .then(
            choice((
                one_of('A'..='F'),
                one_of('a'..='f'),
                one_of('0'..='9'))
            )
            .repeated()
            .at_least(1)
        )
        .to_slice()
        .padded()
        .map(|hexnum| Hexnum(hexnum))
}

#[cfg(test)]
mod tests {

    use super::*;

    // == statements
    #[test]
    fn statements_return() {
        assert_eq!(
            statements_parser().parse("return 0;").into_result(),
            Ok(Statements::Statement {
                current: Statement::Return(Expression::Intconst(Intconst::Decnum(Decnum("0")))),
                next: Box::new(Statements::None)
            })
        )
    }

    #[test]
    fn statements_empty() {
        assert_eq!(
            statements_parser().parse("").into_result(),
            Ok(Statements::None)
        );
    }

    #[test]
    fn statements_empty_padded() {
        assert_eq!(
            statements_parser().parse(" ").into_result(),
            Ok(Statements::None)
        )
    }

    // == statement
    #[test]
    fn statement_simple_return() {
        assert_eq!(
            statement_parser().parse(" return 0; ").into_result(),
            Ok(Statement::Return(Expression::Intconst(Intconst::Decnum(
                Decnum("0")
            ))))
        );
    }

    // == ident
    #[test]
    fn ident_simple() {
        assert_eq!(
            ident_parser().parse("hello").into_result(),
            Ok(Identifier("hello"))
        );
    }

    #[test]
    fn ident_full() {
        assert_eq!(
            ident_parser().parse("AZaz_0").into_result(),
            Ok(Identifier("AZaz_0"))
        );
    }

    #[test]
    fn ident_invalid() {
        assert!(ident_parser().parse("0no").into_result().is_err());
    }

    // == decnum
    #[test]
    fn decnum_parser_zero() {
        assert_eq!(decnum_parser().parse("0").into_result(), Ok(Decnum("0")));
    }

    #[test]
    fn decnum_parser_valid_number() {
        assert_eq!(
            decnum_parser().parse("123").into_result(),
            Ok(Decnum("123"))
        );
    }

    #[test]
    fn decnum_parser_invalid_number() {
        assert!(decnum_parser().parse("0123").into_result().is_err());
    }

    // == hexnum
    #[test]
    fn hexnum_parser_simple() {
        assert_eq!(
            hexnum_parser().parse("0xabc").into_result(),
            Ok(Hexnum("0xabc"))
        );
    }

    #[test]
    fn hexnum_parser_big_x() {
        assert_eq!(
            hexnum_parser().parse("0Xabc").into_result(),
            Ok(Hexnum("0Xabc"))
        );
    }

    #[test]
    fn hexnum_parser_missing_numbers() {
        assert!(hexnum_parser().parse("0x").into_result().is_err());
    }

    // == programs

    #[test]
    fn empty_main() {
        assert_eq!(
            parser().parse("int main() { }").into_result(),
            Ok(Program(Statements::None))
        );
    }

    #[test]
    fn simple_return_main() {
        assert_eq!(
            parser().parse("int main() { return 0; }").into_result(),
            Ok(Program(Statements::Statement {
                current: Statement::Return(Expression::Intconst(Intconst::Decnum(Decnum("0")))),
                next: Box::new(Statements::None)
            }))
        );
    }

    // == sandbox
    #[test]
    fn sandbox() {}
}
