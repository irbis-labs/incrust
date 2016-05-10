
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
}

#[derive(Debug, PartialEq)]
pub struct Mustache {
    pub expr: Expression,
    pub filters: Vec<FilterItem>
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Variable(String),
}

#[derive(Debug, PartialEq)]
pub enum FilterItem {
    Simple(String),
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    cmd: String,
    id: Option<String>,
}

impl Into<Statement> for (String, Option<String>) {
    fn into(self) -> Statement { Statement{cmd: self.0, id: self.1} }
}



// ---------------------------------------------------------------------------

impl Mustache {
    pub fn new(expr: Expression, filters: Vec<FilterItem>) -> Self {
        Mustache { expr: expr, filters: filters }
    }
}

impl From<Mustache> for Parsed {
    fn from(v: Mustache) -> Self { Parsed::Mustache(v) }
}
