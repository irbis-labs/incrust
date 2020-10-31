pub mod ast;
pub mod builder;
pub mod render;

use self::ast::TemplateBlock;
use crate::args::Args;
use crate::template::render::RenderContent;
use crate::template::builder::content::ContentBuilder;

pub struct Template {
    content: Vec<TemplateBlock>,
}

impl Template {
    pub fn builder() -> ContentBuilder<impl FnOnce(Vec<TemplateBlock>) -> Self>  {
        ContentBuilder::new(move |content| Template { content })
    }

    pub fn new(content: Vec<TemplateBlock>) -> Self {
        Template { content }
    }

    pub fn render<'s: 'a, 'a>(&'s self, args: &'a Args) -> RenderContent<'a> {
        RenderContent::new(&self.content, args)
    }
}
