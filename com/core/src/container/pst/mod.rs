#[derive(Debug, Clone, PartialEq)]
pub enum Node<'i> {
    CharLiteral(&'i [u8]),
    Comment(&'i [u8]),
    Identifier(&'i [u8]),
    NumberLiteral(&'i [u8]),
    Statement(
        Box<Node<'i>>, // statement expression.
        bool, // strip_before.
        bool, // strip_after.
    ),
    StatementExpression(Box<Node<'i>>),
    StringLiteral(&'i [u8]),
}

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
