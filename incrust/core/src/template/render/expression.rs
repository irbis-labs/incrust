use std::fmt;

use format::AbstractFilterFactory;

use crate::evaluate::EvalResult;
use crate::template::ast::Expression;
use crate::value::Value;
use crate::Context;

pub struct RenderExpression<'a> {
    pub expression: &'a Expression<'a>,
    pub filters: &'a Vec<Box<dyn AbstractFilterFactory>>,
    pub context: &'a Context<'a>,
}

impl<'a> RenderExpression<'a> {
    pub fn new(
        expression: &'a Expression,
        filters: &'a Vec<Box<dyn AbstractFilterFactory>>,
        context: &'a Context<'a>,
    ) -> Self {
        RenderExpression {
            expression,
            filters,
            context,
        }
    }

    pub fn eval(&self) -> EvalResult<Value<'a>> {
        self.expression.eval(self.context)
    }
}

impl<'a> fmt::Display for RenderExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.eval() {
            Ok(value) => {
                let mut filter: Box<dyn fmt::Display + 'a> = Box::new(value);
                for factory in self.filters {
                    filter = factory.pipe(filter);
                }
                filter.fmt(f)?
            }
            Err(err) => log::debug!("{}", err),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use format::CapitalizeFactory;

    use crate::template::ast::{BinOp, Expression, TemplateBlock as TB};
    use crate::template::Template;
    use crate::value::{Integer, Value};
    use crate::{Args, Incrust};

    #[test]
    fn render_template() {
        let mut args = Args::new();
        args.insert("issue", &"answer");
        args.insert("left", Integer::from(6));
        args.insert("right", Value::from(7));
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
                        expression: Expression::var("issue"),
                        filters: vec![Box::new(CapitalizeFactory)],
                    },
                    TB::PlainText {
                        content: " is: ".to_string(),
                    },
                    TB::Expression {
                        expression: Expression::bin_op(
                            BinOp::Mul,
                            Expression::var("left"),
                            Expression::var("right"),
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
        let env = Incrust::new();

        let sample = "<html><title>The Answer is: 42</title></html>";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result)
    }

    #[test]
    fn render_condition() {
        let mut args = Args::new();
        args.insert("a", true);
        args.insert("b", false);
        let content = vec![
            TB::PlainText {
                content: "True and False: ".to_string(),
            },
            TB::Expression {
                expression: Expression::bin_op(
                    BinOp::And,
                    Expression::var("a"),
                    Expression::var("b"),
                ),
                filters: vec![Box::new(CapitalizeFactory)],
            },
            TB::PlainText {
                content: "\nTrue or False: ".to_string(),
            },
            TB::Expression {
                expression: Expression::bin_op(
                    BinOp::Or,
                    Expression::var("a"),
                    Expression::var("b"),
                ),
                filters: vec![Box::new(CapitalizeFactory)],
            },
            TB::PlainText {
                content: "\n".to_string(),
            },
        ];
        let template = Template::new(content);
        let env = Incrust::new();

        let sample = "True and False: False\nTrue or False: True\n";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result)
    }
}
