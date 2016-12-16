use std::borrow::Cow;

use abc::*;
use incrust::Incrust;
use container::Template;


pub type TemplateStack<'a> = Vec<&'a Template>;


pub struct GlobalContext<'a> {
    env: &'a Incrust,
    stack: Vec<Cow<'a, Template>>,
    args: &'a Args<'a>,
}


pub struct Context<'a> {
    global: &'a GlobalContext<'a>,
    parent: Option<&'a Context<'a>>,
    args: &'a Args<'a>,
}


pub enum ParentScope<'a> {
    Global(&'a GlobalContext<'a>),
    Local(&'a Context<'a>),
}

impl <'a> From<&'a GlobalContext<'a>> for ParentScope<'a> {
    fn from(x: &'a GlobalContext<'a>) -> Self {
        ParentScope::Global(x)
    }
}

impl <'a> From<&'a Context<'a>> for ParentScope<'a> {
    fn from(x: &'a Context<'a>) -> Self {
        ParentScope::Local(x)
    }
}


impl <'a> GlobalContext<'a> {
    pub fn new(env: &'a Incrust, template: &'a Template, args: &'a Args<'a>) -> RenderResult<Self> {
        use ::renderer::evaluator::eval_expr;

        let mut context = GlobalContext {
            env: env,
            stack: vec![Cow::Borrowed(template)],
            args: args,
        };

        loop {
            if let Some(parent) = context.template().extends.clone() {
                let template = {
                    let local = context.top_scope();
                    let name = eval_expr(&local, &parent.expr)?
                        .ok_or(LoadError::BadName("Can't evaluate name (None result)".into()))?;
                    let name = name.try_as_string()
                        .ok_or(LoadError::BadName("Name is not string".into()))?;
                    Cow::Owned(env.parse(&env.load(&name)?)?)
                };
                context.stack.push(template);
            } else {
                break;
            }
        }

        Ok(context)
    }

    pub fn top_scope(&'a self) -> Context<'a> {
        Context::new(self, self.args)
    }

    pub fn template(&'a self) -> &'a Template {
        self.stack.last().as_ref().unwrap()
    }

    pub fn stack(&'a self) -> &'a [Cow<'a, Template>] {
        &self.stack
    }

    pub fn env(&self) -> &'a Incrust {
        self.env
    }
}


impl <'a> Context<'a> {
    pub fn new<PS: Into<ParentScope<'a>>>(parent_scope: PS, args: &'a Args<'a>) -> Self {
        let (global, parent) = match parent_scope.into() {
            ParentScope::Global(global) => (global, None),
            ParentScope::Local(local) => (local.global(), Some(local)),
        };
        Context {
            global: global,
            parent: parent,
            args: args
        }
    }

    pub fn nested_scope(&'a self, args: &'a Args<'a>) -> Self {
        Context {
            global: self.global,
            parent: Some(self),
            args: args
        }
    }

    pub fn template(&self) -> &'a Template {
        self.global.template()
    }

    pub fn global(&self) -> &'a GlobalContext<'a> {
        self.global
    }

    pub fn env(&self) -> &'a Incrust {
        self.global.env()
    }

    pub fn get(&self, id: &str) -> Option<&BType> {
        self.args.get(id)
            .or_else(|| self.parent
                .and_then(|p| p.get(id))
                .or_else(|| self.global.env().top_context().get(id))
            )
    }
}
