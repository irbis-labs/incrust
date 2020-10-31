use std::fmt;

use crate::args::Args;
use crate::template::ast::TemplateBlock;
use crate::template::render::RenderExpression;

pub struct RenderContent<'a> {
    content: &'a [TemplateBlock],
    args: &'a Args,
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
                    write!(f, "{}", content)?;
                }
                TemplateBlock::Block { name, content } => {
                    let render = RenderContent {
                        content: &content,
                        args: self.args,
                    };
                    write!(f, "{}", render)?;
                }
                TemplateBlock::Expression { expression, filters } => {
                    let render = RenderExpression {
                        expression,
                        filters,
                        args: self.args,
                    };
                    write!(f, "{}", render)?;
                }
                // TemplateBlock::Expression { expression } => match expression.evaluate() {
                //     Ok(value) => write!(f, "{}", value.render())?,
                //     Err(err) => write!(f, "<!-- {:?} -->", err)?,
                // },
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::args::Args;
    use crate::template::ast::{Expression, TemplateBlock as TB};
    use crate::template::Template;
    use crate::value::NativeValue;

    // #[test]
    // fn render_template() {
    //     let args = Args {};
    //     let content = vec![
    //         TB::PlainText {
    //             content: "<html>".to_string(),
    //         },
    //         TB::Block {
    //             name: "title".to_string(),
    //             content: vec![
    //                 TB::PlainText {
    //                     content: "<title>".to_string(),
    //                 },
    //                 TB::Expression {
    //                     expression: Expression::value(Value::String("Title".to_string())),
    //                 },
    //                 TB::PlainText {
    //                     content: "</title>".to_string(),
    //                 },
    //             ],
    //         },
    //         TB::PlainText {
    //             content: "</html>".to_string(),
    //         },
    //     ];
    //     let template = Template::new(content);
    //     let sample = "<html><title>Title</title></html>";
    //     let result = template.render(&args).to_string();
    //     assert_eq!(sample, result)
    // }

    #[test]
    fn build_and_render_template() {
        let args = Args {};
        let template = Template::builder()
            .plain_text("<html>")
            .block("title")
            .plain_text("<title>")
            .expression(Expression::value("Title".to_string()), vec![])
            .plain_text("</title>")
            .finish()
            .plain_text("</html>")
            .finish();
        let sample = "<html><title>Title</title></html>";
        let result = template.render(&args).to_string();
        assert_eq!(sample, result)
    }
}
