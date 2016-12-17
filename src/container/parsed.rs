use container::expression::*;


pub type ParsedNodes = Vec<ParsedNode>;


#[derive(Debug, PartialEq, Clone)]
pub enum ParsedNode {
    Raw(ParsedRawStatement),
    Text(String),
    Comment(String),
    Mustache(Mustache),
    For(ParsedForStatement),
    If(ParsedIfStatement),
    Block(ParsedBlockStatement),
    Extends(ExprStatement),
    Include(ExprStatement),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleStatement {
    pub strip_left: bool,
    pub strip_right: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedStatement {
    pub strip_left: bool,
    pub strip_right: bool,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprStatement {
    pub strip_left: bool,
    pub strip_right: bool,
    pub expression: FullExpression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedRawStatement {
    pub begin: SimpleStatement,
    pub text: String,
    pub end: SimpleStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedIfBranch {
    pub begin: ExprStatement,
    pub block: ParsedNodes,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedElseBranch {
    pub begin: SimpleStatement,
    pub block: ParsedNodes,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedIfStatement {
    pub if_branches: Vec<ParsedIfBranch>,
    pub else_branch: Option<ParsedElseBranch>,
    pub end: SimpleStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedForStatement {
    pub begin: ExprStatement,
    pub block: ParsedNodes,
    pub key_var: Option<String>,
    pub value_var: String,
    pub end: SimpleStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParsedBlockStatement {
    pub begin: NamedStatement,
    pub block: ParsedNodes,
    pub end: SimpleStatement,
}

// ---------------------------------------------------------------------------

impl Default for SimpleStatement {
    fn default() -> Self {
        SimpleStatement {
            strip_left: false,
            strip_right: false,
        }
    }
}

impl Default for NamedStatement {
    fn default() -> Self {
        NamedStatement {
            strip_left: false,
            strip_right: false,
            name: Default::default(),
        }
    }
}

//impl Default for ExprStatement {
//    fn default() -> Self {
//        ExprStatement {
//            strip_left: false,
//            strip_right: false,
//            expression: None,
//        }
//    }
//}

impl Mustache { pub fn new(expr: FullExpression) -> Self { Mustache { expr: expr } } }
impl From<Mustache> for ParsedNode { fn from(v: Mustache) -> Self { ParsedNode::Mustache(v) } }


impl FullExpression {
    pub fn new(expr: DisjExpr, filters: Vec<FilterItem>) -> Self {
        FullExpression { expr: expr, filters: filters }
    }
}


impl From<Literal> for Factor { fn from(v: Literal) -> Self { Factor::Literal(v) } }
impl From<String> for Factor { fn from(v: String) -> Self { Factor::Variable(v) } }
impl From<DisjExpr> for Factor { fn from(v: DisjExpr) -> Self { Factor::Subexpression(v) } }
impl From<Attribute> for Factor { fn from(v: Attribute) -> Self { Factor::Attribute(v) } }
impl From<Invocation> for Factor { fn from(v: Invocation) -> Self { Factor::Invocation(v) } }

impl From<ParsedIfStatement> for ParsedNode { fn from(v: ParsedIfStatement) -> Self { ParsedNode::If(v) } }
impl From<ParsedForStatement> for ParsedNode { fn from(v: ParsedForStatement) -> Self { ParsedNode::For(v) } }
impl From<ParsedBlockStatement> for ParsedNode { fn from(v: ParsedBlockStatement) -> Self { ParsedNode::Block(v) } }
impl From<ParsedRawStatement> for ParsedNode { fn from(v: ParsedRawStatement) -> Self { ParsedNode::Raw(v) } }
