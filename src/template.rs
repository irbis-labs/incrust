
use nom::{IResult};

use ::abc;


#[derive(Debug)]
pub struct Template {
    pub parsed: Vec<Parsed>,
}

impl Template {
    pub fn parse(templ: &str) -> abc::ParseResult {
        let parsed = ::parser::text(templ.as_bytes());
        println!(" == parsed == {:?}", &parsed);
        match parsed {
            IResult::Done(_, parsed) => Ok(Template { parsed: parsed }),
            IResult::Error(err) => Err(abc::ParseError::Syntax(format!("{:?}", err))),
            IResult::Incomplete(needed) => Err(abc::ParseError::Syntax(format!("Incomplete; {:?}", needed))),
        }
    }
}

// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum Parsed {
    Text(String),
    Comment(String),
    Mustache(Mustache),
    ForEach(ForEach),
}

#[derive(Debug, PartialEq)]
pub struct Mustache {
    pub expr: FullExpression,
}

#[derive(Debug, PartialEq)]
pub struct FullExpression {
    pub expr: Expr,
    pub filters: Vec<FilterItem>
}


#[derive(Debug, PartialEq, Clone)]
pub enum SumOp {
    Add,
    Sub,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MulOp {
    Mul,
    Div,
    And,
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub sum: Vec<ExprItem>,
}

#[derive(Debug, PartialEq)]
pub struct ExprItem(pub SumOp, pub Term);


#[derive(Debug, PartialEq)]
pub struct Term {
    pub mul: Vec<TermItem>,
}

#[derive(Debug, PartialEq)]
pub struct TermItem(pub MulOp, pub Factor);


#[derive(Debug, PartialEq)]
pub enum Factor {
    Variable(String),
    Literal(Literal),
//    Literal(BType),
    Subexpression(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Char(char),
    Str(String),
    Int(isize),
    Real(f64),
}

#[derive(Debug, PartialEq)]
pub enum FilterItem {
    Simple(String),
}

#[derive(Debug, PartialEq)]
pub struct Statement {

}

#[derive(Debug, PartialEq)]
pub struct ForEach {
    pub begin: Statement,
    pub end: Statement,
    pub expr: FullExpression,
    pub loop_var: String,
}

impl Into<Statement> for () {
    fn into(self) -> Statement { Statement {} }
}

// ---------------------------------------------------------------------------

impl Mustache { pub fn new(expr: FullExpression) -> Self { Mustache { expr: expr } } }
impl From<Mustache> for Parsed { fn from(v: Mustache) -> Self { Parsed::Mustache(v) } }


impl FullExpression {
    pub fn new(expr: Expr, filters: Vec<FilterItem>) -> Self {
        FullExpression { expr: expr, filters: filters }
    }
}


impl From<Literal> for Factor { fn from(v: Literal) -> Self { Factor::Literal(v) } }
impl From<String> for Factor { fn from(v: String) -> Self { Factor::Variable(v) } }
impl From<Expr> for Factor { fn from(v: Expr) -> Self { Factor::Subexpression(v) } }

