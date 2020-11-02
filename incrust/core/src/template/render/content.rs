use std::fmt;

use crate::template::ast::TemplateBlock;
use crate::template::render::{RenderConditional, RenderExpression};
use crate::{Context, EvalError};

pub struct RenderContent<'a> {
    content: &'a [TemplateBlock],
    context: &'a Context<'a>,
}

impl<'a> RenderContent<'a> {
    pub fn new(content: &'a [TemplateBlock], context: &'a Context<'a>) -> Self {
        RenderContent { content, context }
    }
}

impl<'a> fmt::Display for RenderContent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in self.content {
            match block {
                TemplateBlock::PlainText { content } => {
                    content.fmt(f)?;
                }
                TemplateBlock::Include { name } => {
                    match self.context.template(name) {
                        Some(template) => template.render(self.context).fmt(f)?,
                        None => {
                            let err = EvalError::UnknownTemplate;
                            unimplemented!("{:?}", err)
                        }
                    };
                }
                TemplateBlock::Block { name: _, content } => {
                    RenderContent::new(content, self.context).fmt(f)?;
                }
                TemplateBlock::Expression {
                    expression,
                    filters,
                } => {
                    RenderExpression::new(expression, filters, self.context).fmt(f)?;
                }
                TemplateBlock::Conditional(conditional) => {
                    RenderConditional::new(conditional, self.context).fmt(f)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::template::ast::Expression;
    use crate::template::Template;
    use crate::{Args, Incrust};

    #[test]
    fn build_and_render_template() {
        let template = Template::builder()
            .plain_text("<html>")
            .block("title")
            .plain_text("<title>")
            .expression(Expression::value(&"Title"), vec![])
            .plain_text("</title>")
            .finish()
            .plain_text("</html>")
            .finish();
        let sample = "<html><title>Title</title></html>";

        let env = Incrust::new();
        let args = Args::new();
        let context = env.context(&args);

        let result = template.render(&context).to_string();
        assert_eq!(sample, result)
    }
}
