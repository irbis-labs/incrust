use std::fmt;

use crate::args::Args;
use crate::template::ast::TemplateBlock;
use crate::template::render::{RenderConditional, RenderExpression};

pub struct RenderContent<'a> {
    content: &'a [TemplateBlock],
    args: &'a Args<'a>,
}

impl<'a> RenderContent<'a> {
    pub fn new(content: &'a [TemplateBlock], args: &'a Args) -> Self {
        RenderContent { content, args }
    }
}

impl<'a> fmt::Display for RenderContent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in self.content {
            match block {
                TemplateBlock::PlainText { content } => {
                    content.fmt(f)?;
                }
                TemplateBlock::Block { name: _, content } => {
                    RenderContent::new(content, self.args).fmt(f)?;
                }
                TemplateBlock::Expression {
                    expression,
                    filters,
                } => {
                    RenderExpression::new(expression, filters, self.args).fmt(f)?;
                }
                TemplateBlock::Conditional(conditional) => {
                    RenderConditional::new(conditional, self.args).fmt(f)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::args::Args;
    use crate::template::ast::Expression;
    use crate::template::Template;

    #[test]
    fn build_and_render_template() {
        let args = Args::new();
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
        let result = template.render(&args).to_string();
        assert_eq!(sample, result)
    }
}
