use format::AbstractFilterFactory;

use crate::template::ast::{Expression, TemplateBlock};
use crate::Identifier;

pub struct ContentBuilder<Fun> {
    complete: Fun,
    content: Vec<TemplateBlock>,
}

impl<Fun, Parent> ContentBuilder<Fun>
where
    Fun: FnOnce(Vec<TemplateBlock>) -> Parent,
{
    pub fn new(complete: Fun) -> Self {
        ContentBuilder {
            complete,
            content: Vec::new(),
        }
    }

    pub fn push(mut self, block: TemplateBlock) -> Self {
        self.content.push(block);
        self
    }

    pub fn plain_text(self, text: impl Into<String>) -> Self {
        let content = text.into();
        self.push(TemplateBlock::PlainText { content })
    }

    pub fn block(
        self,
        name: impl Into<Identifier>,
    ) -> ContentBuilder<impl FnOnce(Vec<TemplateBlock>) -> Self> {
        ContentBuilder::new(move |content| {
            let name = name.into();
            let block = TemplateBlock::Block { name, content };
            self.push(block)
        })
    }

    pub fn expression(
        self,
        expression: Expression<'static>,
        filters: Vec<Box<dyn AbstractFilterFactory>>,
    ) -> Self {
        self.push(TemplateBlock::Expression {
            expression,
            filters,
        })
    }

    // pub fn conditional(
    //     self,
    //     condition: Expression<'static>,
    // ) -> ContentBuilder<impl FnOnce(Vec<TemplateBlock>) -> Self> {
    //     ContentBuilder::new(move |content| {
    //         let name = name.into();
    //         let block = TemplateBlock::Block { name, content };
    //         self.push(block)
    //     })
    // }

    pub fn finish(self) -> Parent {
        (self.complete)(self.content)
    }
}
