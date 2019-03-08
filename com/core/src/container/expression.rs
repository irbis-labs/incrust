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
    pub list: Vec<ConjExpr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConjExpr {
    pub list: Vec<CmpExpr>,
}

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
    Invocation(Invocation),
    Index(Index),
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
pub struct Invocation {
    pub args: Vec<DisjExpr>,
    pub on: Box<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Index {
    pub index: Box<DisjExpr>,
    pub on: Box<Factor>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Char(char),
    Str(String),
    Int(i64),
    Real(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FilterItem {
    Simple(String),
}

