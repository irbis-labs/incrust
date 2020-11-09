pub mod ast;
pub mod builder;
pub mod render;

use crate::template::ast::{Expression, TemplateBlock};
use crate::template::builder::content::ContentBuilder;
use crate::template::render::{RenderContent, RenderExtension, RenderTemplate};
use crate::Context;

pub struct Template {
    extends: Option<Expression<'static>>,
    content: Vec<TemplateBlock>,
}

impl Template {
    pub fn builder() -> ContentBuilder<impl FnOnce(Vec<TemplateBlock>) -> Self> {
        ContentBuilder::new(move |content| Template::new(None, content))
    }

    pub fn new(extends: Option<Expression<'static>>, content: Vec<TemplateBlock>) -> Self {
        Template { extends, content }
    }

    pub fn render<'s: 'a, 'a>(&'s self, context: &'a Context<'a>) -> RenderTemplate<'a> {
        match &self.extends {
            Some(extends) => {
                let render = RenderExtension::new(extends, &self.content, context);
                RenderTemplate::Extension(render)
            }
            None => {
                let render = RenderContent::new(&self.content, context);
                RenderTemplate::Content(render)
            }
        }
    }
}
