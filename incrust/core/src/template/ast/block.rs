use format::AbstractFilterFactory;

use crate::template::ast::Expression;
use crate::Identifier;

pub enum TemplateBlock {
    PlainText {
        content: String,
    },
    Include {
        name: Identifier,
    },
    Block {
        name: Identifier,
        content: Vec<TemplateBlock>,
    },
    Expression {
        expression: Expression<'static>,
        filters: Vec<Box<dyn AbstractFilterFactory>>,
    },
    Conditional(Conditional),
}

pub struct Conditional {
    pub branches: Vec<Branch>,
    pub fallback: Vec<TemplateBlock>,
}

pub struct Branch {
    pub condition: Expression<'static>,
    pub content: Vec<TemplateBlock>,
}
