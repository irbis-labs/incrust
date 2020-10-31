use std::fmt;

use format::AbstractFilterFactory;

use crate::args::Args;
use crate::evaluate::{EvalError, EvalResult};
use crate::template::ast::Expression;
use crate::value::Value;

pub struct RenderExpression<'a> {
    pub expression: &'a Expression<'a>,
    pub filters: &'a Vec<Box<dyn AbstractFilterFactory>>,
    pub args: &'a Args,
}

impl<'a> RenderExpression<'a> {
    pub fn new(expression: &'a Expression, filters: &'a Vec<Box<dyn AbstractFilterFactory>>, args: &'a Args) -> Self {
        RenderExpression {
            expression,
            filters,
            args,
        }
    }

    pub fn eval(&self) -> EvalResult<Value<'a>> {
        self.expression.eval(&self.args)
    }
}

impl<'a> fmt::Display for RenderExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.eval();
        match value {
            Ok(value) => {
                let mut filter: Box<dyn fmt::Display + 'a> = Box::new(value);
                for f in self.filters {
                    filter = f.pipe(filter);
                }
                filter.fmt(f)?
            },
            Err(err) => log::debug!("{}", err),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use format::CapitalizeFactory;

    use crate::args::Args;
    use crate::template::ast::{BinOp, Expression, TemplateBlock as TB};
    use crate::template::Template;
    use crate::value::{Integer, NativeValue};

    #[test]
    fn render_template() {
        let args = Args {};
        let content = vec![
            TB::PlainText {
                content: "<html>".to_string(),
            },
            TB::Block {
                name: "title".to_string(),
                content: vec![
                    TB::PlainText {
                        content: "<title>The ".to_string(),
                    },
                    TB::Expression {
                        expression: Expression::value("answer".to_string()),
                        filters: vec![Box::new(CapitalizeFactory)],
                    },
                    TB::PlainText {
                        content: " is: ".to_string(),
                    },
                    TB::Expression {
                        expression: Expression::bin_op(
                            BinOp::Mul,
                            Expression::value(Integer::from(6)),
                            Expression::value(Integer::from(7)),
                        ),
                        filters: vec![],
                    },
                    TB::PlainText {
                        content: "</title>".to_string(),
                    },
                ],
            },
            TB::PlainText {
                content: "</html>".to_string(),
            },
        ];
        let template = Template::new(content);
        let sample = "<html><title>The Answer is: 42</title></html>";
        let result = template.render(&args).to_string();
        assert_eq!(sample, result)
    }
}
