use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    IncorrectCharLiteral,
    IncorrectNumberLiteral,
    IncorrectStatement,
    IncorrectStringLiteral,
    NomCode(u32),
    NotRecognized,
    UnclosedCharLiteral,
    UnclosedComment,
    UnclosedStatement,
    UnclosedStringLiteral,
}

impl From<u32> for ErrorKind {
    fn from(code: u32) -> Self {
        ErrorKind::NomCode(code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharLiteral<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementExpression<'i>(pub Identifier<'i>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement<'i>{
    pub expression: StatementExpression<'i>,
    pub strip_left: bool,
    pub strip_right: bool,
}

impl<'i> Statement<'i> {
    pub fn new(expression: StatementExpression<'i>, strip_left: bool, strip_right: bool) -> Self {
        Statement { expression, strip_left, strip_right }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOperator {
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    In,
    NotIn,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum Operand<'i> {
    CharLiteral(CharLiteral<'i>),
    Identifier(Identifier<'i>),
    NumberLiteral(NumberLiteral<'i>),
    StringLiteral(StringLiteral<'i>),
    Prefix(PrefixOperator, Box<Operand<'i>>),
}
