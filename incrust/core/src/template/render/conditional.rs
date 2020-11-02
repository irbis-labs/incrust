use std::fmt;

use crate::template::ast::{Conditional, Expression};
use crate::template::render::RenderContent;
use crate::Context;
use crate::{EvalError, EvalResult};

pub struct RenderConditional<'a> {
    block: &'a Conditional,
    context: &'a Context<'a>,
}

impl<'a> RenderConditional<'a> {
    pub fn new(block: &'a Conditional, context: &'a Context<'a>) -> Self {
        RenderConditional { block, context }
    }

    fn eval_condition(&self, condition: &Expression<'static>) -> EvalResult<bool> {
        let result = condition.eval(self.context)?;
        Ok(result.to_boolean().ok_or(EvalError::BooleanExpected)?)
    }
}

impl<'a> fmt::Display for RenderConditional<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for branch in &self.block.branches {
            match self.eval_condition(&branch.condition) {
                Ok(result) => {
                    if result {
                        RenderContent::new(&branch.content, self.context).fmt(f)?;
                        return Ok(());
                    }
                }
                Err(err) => log::debug!("{}", err),
            }
        }
        RenderContent::new(&self.block.fallback, self.context).fmt(f)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::template::ast::{Branch, Conditional, Expression, TemplateBlock as TB};
    use crate::template::Template;
    use crate::{Args, Incrust};

    #[test]
    fn build_and_render_template() {
        let mut args = Args::new();
        let content = vec![
            TB::PlainText {
                content: "Result: ".to_string(),
            },
            TB::Conditional(Conditional {
                branches: vec![Branch {
                    condition: Expression::var("result"),
                    content: vec![TB::PlainText {
                        content: "yes".to_string(),
                    }],
                }],
                fallback: vec![TB::PlainText {
                    content: "no".to_string(),
                }],
            }),
        ];
        let template = Template::new(content);
        let env = Incrust::new();

        args.insert("result", true);
        let sample = "Result: yes";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result);

        args.insert("result", false);
        let sample = "Result: no";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result);
    }
}
