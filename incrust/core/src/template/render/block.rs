use std::fmt;

use crate::template::ast::TemplateBlock;
use crate::template::render::RenderContent;
use crate::{Args, Context, Identifier};

pub struct RenderBlock<'a> {
    name: &'a Identifier,
    parent: Option<&'a RenderBlock<'a>>,
    content: &'a [TemplateBlock],
    context: &'a Context<'a>,
}

impl<'a> RenderBlock<'a> {
    pub fn new(
        name: &'a Identifier,
        content: &'a [TemplateBlock],
        context: &'a Context<'a>,
    ) -> Self {
        let parent = None;
        RenderBlock {
            name,
            parent,
            content,
            context,
        }
    }

    pub fn with_parent(
        parent: &'a RenderBlock<'a>,
        content: &'a [TemplateBlock],
        context: &'a Context<'a>,
    ) -> Self {
        let name = parent.name;
        let parent = Some(parent);
        RenderBlock {
            name,
            parent,
            content,
            context,
        }
    }

    pub fn child(&'a self, content: &'a [TemplateBlock], context: &'a Context<'a>) -> Self {
        Self::with_parent(self, content, context)
    }

    fn render_back(&'a self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Display;

        if let Some(parent) = self.parent {
            let mut found = None;
            for block in self.content {
                if let TemplateBlock::Block { name, content } = block {
                    if name == self.name {
                        found = Some(content);
                        break;
                    }
                }
            }

            if let Some(content) = found {
                let args = Args::new();
                // TODO context should be created and passed in the render function.
                // args.insert("super", Value::InvocableRef(&make_parent_render));
                let context = self.context.push(Some(&args), None);
                RenderContent::new(content, &context).fmt(f)?;
            } else {
                parent.render_back(f)?;
            }
        } else {
            RenderContent::new(self.content, self.context).fmt(f)?;
        }
        Ok(())
    }
}

impl<'a> fmt::Display for RenderBlock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(child) = self.context.child() {
            self.child(child.content(), child.context()).fmt(f)?;
        } else {
            self.render_back(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::template::ast::Expression;
    use crate::template::Template;
    use crate::template::TemplateBlock as TB;
    use crate::{Args, Identifier, Incrust};

    #[test]
    fn extends_2() {
        let parent = Template::new(
            None,
            vec![
                TB::PlainText {
                    content: "<title>".to_string(),
                },
                TB::Block {
                    name: Identifier::new("title"),
                    content: vec![TB::PlainText {
                        content: "global title".to_string(),
                    }],
                },
                TB::PlainText {
                    content: "</title>".to_string(),
                },
            ],
        );
        let child_local = Template::new(
            Some(Expression::value(&"parent")),
            vec![TB::Block {
                name: Identifier::new("title"),
                content: vec![TB::PlainText {
                    content: "local title".to_string(),
                }],
            }],
        );
        let child_default = Template::new(Some(Expression::value(&"parent")), vec![]);
        let mut env = Incrust::new();
        env.register_template("parent", parent).unwrap();

        let args = Args::new();
        let context = env.context(&args);
        let sample = "<title>local title</title>";
        let result = child_local.render(&context).to_string();
        assert_eq!(sample, result);

        let sample = "<title>global title</title>";
        let result = child_default.render(&context).to_string();
        assert_eq!(sample, result);
    }

    #[test]
    fn extends_3() {
        let site = Template::new(
            None,
            vec![
                TB::PlainText {
                    content: "<title>".to_string(),
                },
                TB::Block {
                    name: Identifier::new("title"),
                    content: vec![TB::PlainText {
                        content: "site title".to_string(),
                    }],
                },
                TB::PlainText {
                    content: "</title>".to_string(),
                },
            ],
        );
        let section_replace = Template::new(
            Some(Expression::value(&"site")),
            vec![TB::Block {
                name: Identifier::new("title"),
                content: vec![TB::PlainText {
                    content: "section title".to_string(),
                }],
            }],
        );
        let section_noop = Template::new(Some(Expression::value(&"site")), vec![]);
        let page_local1 = Template::new(
            Some(Expression::value(&"section_replace")),
            vec![TB::Block {
                name: Identifier::new("title"),
                content: vec![TB::PlainText {
                    content: "page title".to_string(),
                }],
            }],
        );
        let page_local2 = Template::new(
            Some(Expression::value(&"section_noop")),
            vec![TB::Block {
                name: Identifier::new("title"),
                content: vec![TB::PlainText {
                    content: "page title".to_string(),
                }],
            }],
        );
        let page_section = Template::new(Some(Expression::value(&"section_replace")), vec![]);
        let page_site = Template::new(Some(Expression::value(&"section_noop")), vec![]);

        let mut env = Incrust::new();
        env.register_template("site", site).unwrap();
        env.register_template("section_replace", section_replace)
            .unwrap();
        env.register_template("section_noop", section_noop).unwrap();

        let args = Args::new();
        let context = env.context(&args);

        let sample = "<title>page title</title>";
        let result = page_local1.render(&context).to_string();
        assert_eq!(sample, result);
        let result = page_local2.render(&context).to_string();
        assert_eq!(sample, result);

        let sample = "<title>section title</title>";
        let result = page_section.render(&context).to_string();
        assert_eq!(sample, result);

        let sample = "<title>site title</title>";
        let result = page_site.render(&context).to_string();
        assert_eq!(sample, result);
    }
}
