
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
    pub expr: Expression,
    pub filters: Vec<FilterItem>
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(String),
    Literal(Literal),
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
    pub expr: Expression,
    pub loop_var: String,
}

impl Into<Statement> for () {
    fn into(self) -> Statement { Statement {} }
}

// ---------------------------------------------------------------------------

impl Mustache {
    pub fn new(expr: FullExpression) -> Self {
        Mustache { expr: expr }
    }
}

impl From<Mustache> for Parsed {
    fn from(v: Mustache) -> Self { Parsed::Mustache(v) }
}


impl FullExpression {
    pub fn new(expr: Expression, filters: Vec<FilterItem>) -> Self {
        FullExpression { expr: expr, filters: filters }
    }
}

impl From<Literal> for Expression {
    fn from(v: Literal) -> Self { Expression::Literal(v) }
}

