use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    IncorrectCharLiteral,
    IncorrectNumberLiteral,
    IncorrectExpression,
    IncorrectStatement,
    IncorrectStringLiteral,
    NomCode(u32),
    NotRecognized,
    UnclosedCharLiteral,
    UnclosedComment,
    UnclosedExpressionTag,
    UnclosedOperation,
    UnclosedStatementTag,
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
pub struct ExpressionTag<'i>{
    pub expression: Operations<'i>,
    pub strip_before: bool,
    pub strip_after: bool,
}

impl<'i> ExpressionTag<'i> {
    pub fn new(expression: Operations<'i>, strip_before: bool, strip_after: bool) -> Self {
        ExpressionTag { expression, strip_before, strip_after }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'i>(pub &'i [u8]);

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

impl InfixOperator {
    pub fn is_keyword(&self) -> bool {
        use self::InfixOperator::*;
        match self {
            And | Or | Xor | In | NotIn => true,
            Eq | NotEq | Gt | Gte | Lt | Lte | Add | Sub | Mul | Div | Mod => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum Operand<'i> {
    CharLiteral(CharLiteral<'i>),
    Identifier(Identifier<'i>),
    NumberLiteral(NumberLiteral<'i>),
    StringLiteral(StringLiteral<'i>),
    Prefix(PrefixOperator, Box<Operand<'i>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operations<'i>(pub Operand<'i>, pub Vec<(InfixOperator, Operand<'i>)>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainText<'i>(pub &'i [u8]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    Minus,
    Not,
}

impl PrefixOperator {
    pub fn is_keyword(&self) -> bool {
        use self::PrefixOperator::*;
        match self {
            Not => true,
            Minus => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementExpression<'i>(pub Identifier<'i>, pub Option<Operations<'i>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementTag<'i>{
    pub expression: StatementExpression<'i>,
    pub strip_before: bool,
    pub strip_after: bool,
}

impl<'i> StatementTag<'i> {
    pub fn new(expression: StatementExpression<'i>, strip_before: bool, strip_after: bool) -> Self {
        StatementTag { expression, strip_before, strip_after }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral<'i>(pub &'i [u8]);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Template<'i>(pub Vec<TemplatePart<'i>>);

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum TemplatePart<'i>{
    Comment(Comment<'i>),
    ExpressionTag(ExpressionTag<'i>),
    PlainText(PlainText<'i>),
    StatementTag(StatementTag<'i>),
}
