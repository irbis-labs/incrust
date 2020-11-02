pub mod ast;
pub mod builder;
pub mod render;

use crate::template::ast::TemplateBlock;
use crate::template::builder::content::ContentBuilder;
use crate::template::render::RenderContent;
use crate::Context;

pub struct Template {
    content: Vec<TemplateBlock>,
}

impl Template {
    pub fn builder() -> ContentBuilder<impl FnOnce(Vec<TemplateBlock>) -> Self> {
        ContentBuilder::new(move |content| Template { content })
    }

    pub fn new(content: Vec<TemplateBlock>) -> Self {
        Template { content }
    }

    pub fn render<'s: 'a, 'a>(&'s self, context: &'a Context<'a>) -> RenderContent<'a> {
        RenderContent::new(&self.content, context)
    }
}
