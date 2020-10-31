use format::AbstractFilterFactory;

use crate::template::ast::Expression;

pub enum TemplateBlock {
    PlainText {
        content: String,
    },
    Block {
        name: String,
        content: Vec<TemplateBlock>,
    },
    Expression {
        expression: Expression<'static>,
        filters: Vec<Box<dyn AbstractFilterFactory>>,
    }
}
