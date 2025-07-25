/// aka: The AST
#[derive(Debug, Clone, PartialEq)]
pub struct Program<'a>(pub Statements<'a>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statements<'a> {
    None,
    Statement {
        current: Statement<'a>,
        next: Box<Statements<'a>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Decl(Declaration<'a>),
    Simp(SimpleInstruction<'a>),
    Return(Expression<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration<'a> {
    Ident(Identifier<'a>),
    IdentExp {
        ident: Identifier<'a>,
        exp: Expression<'a>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleInstruction<'a> {
    pub lvalue: LValue<'a>,
    pub asnop: AsNop,
    pub exp: Expression<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LValue<'a> {
    Ident(Identifier<'a>),
    LValue(Box<LValue<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Exp(Box<Expression<'a>>),
    Intconst(Intconst<'a>),
    Ident(Identifier<'a>),
    Binop {
        left: Box<Expression<'a>>,
        op: BinOperation,
        right: Box<Expression<'a>>,
    },
    Unop {
        op: UnOperation,
        right: Box<Expression<'a>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Intconst<'a> {
    Decnum(Decnum<'a>),
    Hexnum(Hexnum<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOperation {
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsNop {
    Equal,
    PlusEqual,
    MinusEqual,
    MultEqual,
    DivEqual,
    ModEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOperation {
    Plus,
    Minus,
    Multiplication,
    Division,
    Mod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq)]
pub struct Decnum<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq)]
pub struct Hexnum<'a>(pub &'a str);
