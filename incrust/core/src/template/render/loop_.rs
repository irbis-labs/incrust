use std::fmt;

use crate::template::ast::Loop;
use crate::template::render::RenderContent;
use crate::{Args, Context, EvalError, RenderResult, Value};

pub struct RenderLoop<'a> {
    block: &'a Loop,
    context: &'a Context<'a>,
}

impl<'a> RenderLoop<'a> {
    pub fn new(block: &'a Loop, context: &'a Context<'a>) -> Self {
        RenderLoop { block, context }
    }

    fn render(&self, f: &mut fmt::Formatter<'_>) -> RenderResult<()> {
        use std::fmt::Display;

        let iterator = self.block.expression.eval(self.context)?;
        let iterator = iterator
            .as_native()
            .and_then(|v| v.to_iterator())
            .ok_or(EvalError::BooleanExpected)?;

        let mut first = true;
        let mut index: i64 = 1;

        for value in iterator {
            let mut args = Args::new();
            args.insert(self.block.var.clone(), value);
            // TODO the loop variable:
            //  * loop.index
            //  * loop.index0
            //  * loop.first
            //  * loop.last
            //  * loop.odd
            //  * loop.even

            // TODO correct lifetimes?
            // args.insert("loop_first", first);

            args.insert("loop_first", Value::from(first));
            args.insert("loop_index", Value::from(index));

            let context = self.context.push(Some(&args), None);
            RenderContent::new(&self.block.content, &context).fmt(f)?;

            first = false;
            index += 1;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for RenderLoop<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Err(err) = self.render(f) {
            log::debug!("{}", err)
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use format::CapitalizeFactory;

    use crate::template::ast::{Branch, Conditional, Expression, Loop, TemplateBlock as TB};
    use crate::template::Template;
    use crate::{Args, Incrust, Value};

    #[test]
    fn render_template() {
        let content = vec![
            TB::PlainText {
                content: "List: ".to_string(),
            },
            TB::Loop(Loop {
                var: "i".into(),
                expression: Expression::var("list"),
                content: vec![
                    TB::Conditional(Conditional {
                        branches: vec![Branch {
                            condition: Expression::var("loop_first"),
                            content: vec![],
                        }],
                        fallback: vec![TB::PlainText {
                            content: ", ".to_string(),
                        }],
                    }),
                    TB::Expression {
                        expression: Expression::var("loop_index"),
                        filters: vec![],
                    },
                    TB::PlainText {
                        content: ") ".to_string(),
                    },
                    TB::Expression {
                        expression: Expression::var("i"),
                        filters: vec![Box::new(CapitalizeFactory)],
                    },
                ],
            }),
            TB::PlainText {
                content: ".".to_string(),
            },
        ];
        let template = Template::new(None, content);
        let env = Incrust::new();

        // ----

        let list = ["apple", "banana"].as_ref();

        let mut args = Args::new();
        args.insert("list", &list);

        let sample = "List: 1) Apple, 2) Banana.";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result);

        // ----

        let vec = vec!["apple", "banana"];
        let slice = vec.as_slice();

        let mut args = Args::new();
        args.insert("list", &slice);

        let sample = "List: 1) Apple, 2) Banana.";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result);

        // ----

        let vec = vec!["apple", "banana"];

        let mut args = Args::new();
        args.insert("list", vec);

        let sample = "List: 1) Apple, 2) Banana.";
        let result = template.render(&env.context(&args)).to_string();
        assert_eq!(sample, result);
    }
}
