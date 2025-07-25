use crate::grammar::*;
use chumsky::{Parser, prelude::*};

pub fn parse<'a>(code: &'a str) -> Result<Program<'a>, String> {
    todo!()
}

fn parser<'src>() -> impl Parser<'src, &'src str, Program<'src>> {
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

fn statements_parser<'src>() -> impl Parser<'src, &'src str, Statements<'src>> {
    let none = empty().to(Statements::None);
    let statements = recursive(|statements_parser| {
        statement_parser()
            .boxed()
            .then(statements_parser)
            .map(|(statement, next)| Statements::Statement {
                current: statement,
                next: Box::new(next),
            })
    });

    choice((none, statements))
}

fn statement_parser<'src>() -> impl Parser<'src, &'src str, Statement<'src>> {
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
        .then_ignore(just(';'))
        .map(|(_, exp)| Statement::Return(exp));

    choice((decl, simp, ret))
}

fn decl_parser<'src>() -> impl Parser<'src, &'src str, Declaration<'src>> {
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

    choice((init, decl))
}

fn simp_parser<'src>() -> impl Parser<'src, &'src str, SimpleInstruction<'src>> {
    lvalue_parser()
        .then(asnop_parser())
        .then(exp_parser())
        .map(|((lvalue, asnop), exp)| SimpleInstruction { lvalue, asnop, exp })
}

fn lvalue_parser<'src>() -> impl Parser<'src, &'src str, LValue<'src>> {
    let ident = ident_parser().map(|ident| LValue::Ident(ident));
    let lvalue = recursive(|lvalue_parser| {
        just('(')
            .ignored()
            .padded()
            .then(lvalue_parser)
            .then_ignore(just(')').padded())
            .map(|(_, lvalue)| LValue::LValue(Box::new(lvalue)))
    });

    choice((ident, lvalue))
}

fn exp_parser<'src>() -> impl Parser<'src, &'src str, Expression<'src>> {
    let nested_exp = recursive(|exp_parser| {
        just('(')
            .ignored()
            .padded()
            .then(exp_parser)
            .then_ignore(just(')').padded())
            .map(|(_, e)| Expression::Exp(Box::new(e)))
    });

    let intconst = intconst_parser().map(|i| Expression::Intconst(i));
    let ident = ident_parser().map(|ident| Expression::Ident(ident));

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

    choice((nested_exp, intconst, ident, binop_exp, unop_exp))
}

fn intconst_parser<'src>() -> impl Parser<'src, &'src str, Intconst<'src>> {
    let dec = decnum_parser().map(|d| Intconst::Decnum(d));
    let hex = hexnum_parser().map(|h| Intconst::Hexnum(h));

    dec.or(hex)
}

fn unop_parser<'src>() -> impl Parser<'src, &'src str, UnOperation> {
    just('-').to(UnOperation::Minus)
}

fn asnop_parser<'src>() -> impl Parser<'src, &'src str, AsNop> {
    let equal = just("=").to(AsNop::Equal);
    let plus = just("+=").to(AsNop::PlusEqual);
    let minus = just("-=").to(AsNop::MinusEqual);
    let mul = just("*=").to(AsNop::MultEqual);
    let div = just("/=").to(AsNop::DivEqual);
    let r#mod = just("%=").to(AsNop::ModEqual);

    choice((equal, plus, minus, mul, div, r#mod))
}

fn binop_parser<'src>() -> impl Parser<'src, &'src str, BinOperation> {
    one_of("+-*/%").map(|op| match op {
        '+' => BinOperation::Plus,
        '-' => BinOperation::Minus,
        '*' => BinOperation::Multiplication,
        '/' => BinOperation::Division,
        '%' => BinOperation::Mod,
        _ => unreachable!("Wallah Krise"),
    })
}

fn ident_parser<'src>() -> impl Parser<'src, &'src str, Identifier<'src>> {
    #[rustfmt::skip]
    let prefix = {
        choice( (
            one_of('A'..'Z'),
            one_of('a'..'z'),
            just('_')
        ))
        };

    let suffix = choice((
        one_of('A'..'Z'),
        one_of('a'..'z'),
        one_of('0'..'9'),
        just('_'),
    ))
    .repeated();

    prefix
        .then(suffix)
        .to_slice()
        .map(|ident| Identifier(ident))
}

fn decnum_parser<'src>() -> impl Parser<'src, &'src str, Decnum<'src>> {
    let decnum = one_of('1'..'9')
        .then(one_of('0'..'9').repeated())
        .to_slice()
        .map(|dec| Decnum(dec));
    let just_zero = just("0").map(|z| Decnum(z));

    decnum.or(just_zero)
}

#[rustfmt::skip]
fn hexnum_parser<'src>() -> impl Parser<'src, &'src str, Hexnum<'src>> {
    just('0')
        .then(one_of("xX"))
        .then(
            choice((
                one_of('A'..'F'),
                one_of('a'..'f'),
                one_of('0'..'9'))
            )
            .repeated()
            .at_least(1)
        )
        .to_slice()
        .map(|hexnum| Hexnum(hexnum))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_main() {
        assert_eq!(
            parser().parse("int main() { }").into_result(),
            Ok(Program(Statements::None))
        );
    }
}
