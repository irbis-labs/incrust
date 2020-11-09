use std::fmt;

use crate::template::ast::{Expression, TemplateBlock};
use crate::{Context, Identifier};

pub struct RenderExtension<'a> {
    extends: &'a Expression<'static>,
    content: &'a [TemplateBlock],
    context: &'a Context<'a>,
}

impl<'a> RenderExtension<'a> {
    pub fn new(
        extends: &'a Expression<'static>,
        content: &'a [TemplateBlock],
        context: &'a Context<'a>,
    ) -> Self {
        RenderExtension {
            extends,
            content,
            context,
        }
    }

    pub fn content(&'a self) -> &'a [TemplateBlock] {
        self.content
    }

    pub fn context(&'a self) -> &'a Context<'a> {
        self.context
    }
}

impl<'a> fmt::Display for RenderExtension<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let extends = match self.extends.eval(self.context) {
            Ok(v) => v,
            Err(e) => {
                write!(f, "Failed to evaluate `extends`: {:?}", e)?;
                return Ok(());
            }
        };
        let parent_id = Identifier::new(extends.to_string());
        let parent = match self.context.template(&parent_id) {
            Some(v) => v,
            None => {
                write!(f, "Parent template not found: {:?}", parent_id)?;
                return Ok(());
            }
        };
        let new_context = self.context.extends(&self);
        parent.render(&new_context).fmt(f)?;
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
