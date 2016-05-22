
use nom::{IResult};

use ::abc;


#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Parsed {
    Text(String),
    Comment(String),
    Mustache(Mustache),
    For(ForStatement),
    If(IfStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Mustache {
    pub expr: FullExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FullExpression {
    pub expr: DisjExpr,
    pub filters: Vec<FilterItem>
}


#[derive(Debug, PartialEq, Clone)]
pub enum ConjOp {
    And,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DisjOp {
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CmpOp {
    Lt,
    Lte,
    Eq,
    Neq,
    In,
    Nin,
    Gte,
    Gt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SumOp {
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MulOp {
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisjExpr {
    pub list: Vec<DisjItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DisjItem(pub DisjOp, pub ConjExpr);


#[derive(Debug, PartialEq, Clone)]
pub struct ConjExpr {
    pub list: Vec<ConjItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConjItem(pub ConjOp, pub CmpExpr);


#[derive(Debug, PartialEq, Clone)]
pub struct CmpExpr {
    pub list: Vec<CmpItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CmpItem(pub CmpOp, pub Expr);


#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    pub sum: Vec<ExprItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprItem(pub SumOp, pub Term);


#[derive(Debug, PartialEq, Clone)]
pub struct Term {
    pub mul: Vec<TermItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TermItem(pub MulOp, pub Factor);


#[derive(Debug, PartialEq, Clone)]
pub enum Factor {
    Attribute(Attribute),
    Variable(String),
    Literal(Literal),
    Subexpression(DisjExpr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub id: String,
    pub on: Box<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Char(char),
    Str(String),
    Int(isize),
    Real(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FilterItem {
    Simple(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub strip_left: bool,
    pub strip_right: bool,
    pub expression: Option<FullExpression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub begin: Statement,
    pub block: Template,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub if_branches: Vec<IfBranch>,
    pub else_branch: Option<IfBranch>,
    pub end: Statement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub begin: Statement,
    pub block: Template,
    pub key_var: Option<String>,
    pub value_var: String,
    pub end: Statement,
}

//impl Into<Statement> for () {
//    fn into(self) -> Statement { Statement::default() }
//}

// ---------------------------------------------------------------------------

impl Default for Statement {
    fn default() -> Self {
        Statement {
            strip_left: false,
            strip_right: false,
            expression: None,
        }
    }
}

impl Mustache { pub fn new(expr: FullExpression) -> Self { Mustache { expr: expr } } }
impl From<Mustache> for Parsed { fn from(v: Mustache) -> Self { Parsed::Mustache(v) } }


impl FullExpression {
    pub fn new(expr: DisjExpr, filters: Vec<FilterItem>) -> Self {
        FullExpression { expr: expr, filters: filters }
    }
}


impl From<Literal> for Factor { fn from(v: Literal) -> Self { Factor::Literal(v) } }
impl From<String> for Factor { fn from(v: String) -> Self { Factor::Variable(v) } }
impl From<DisjExpr> for Factor { fn from(v: DisjExpr) -> Self { Factor::Subexpression(v) } }
impl From<Attribute> for Factor { fn from(v: Attribute) -> Self { Factor::Attribute(v) } }

impl From<IfStatement> for Parsed { fn from(v: IfStatement) -> Self { Parsed::If(v) } }
impl From<ForStatement> for Parsed { fn from(v: ForStatement) -> Self { Parsed::For(v) } }

