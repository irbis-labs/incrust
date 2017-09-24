use std::borrow::Cow;

use abc::*;
use {Args, Arg, Incrust, Template};


pub type TemplateStack<'a> = Vec<&'a Template>;


pub struct Stack<'a> {
    env: &'a Incrust,
    template_stack: Vec<Cow<'a, Template>>,
    args: &'a Args<'a>,
}


pub struct VarContext<'a> {
    global: &'a Stack<'a>,
    parent: Option<&'a VarContext<'a>>,
    args: &'a Args<'a>,
}


impl <'a> Stack<'a> {
    pub fn new(env: &'a Incrust, template: &'a Template, args: &'a Args<'a>) -> RenderResult<Self> {
        use ::renderer::evaluator::eval_expr;

        let mut context = Stack {
            env: env,
            template_stack: vec![Cow::Borrowed(template)],
            args: args,
        };

        while let Some(parent) = context.template().extends.clone() {
            let template = {
                let local = context.top_scope();
                let name = eval_expr(&local, &parent.expr)?
                    .ok_or(LoadError::BadName("Can't evaluate name (None result)".into()))?;
                let name = name.try_as_string()
                    .ok_or(LoadError::BadName("Name is not string".into()))?;
                Cow::Owned(env.parse(&env.load(&name)?)?)
            };
            context.template_stack.push(template);
        }

        Ok(context)
    }

    pub fn top_scope(&'a self) -> VarContext<'a> {
         VarContext::new(self, self.args)
    }

    pub fn template(&'a self) -> &'a Template {
        self.template_stack.last().unwrap()
    }

    pub fn stack(&'a self) -> &'a [Cow<'a, Template>] {
        &self.template_stack
    }

    pub fn env(&self) -> &'a Incrust {
        self.env
    }
}


impl <'a> VarContext<'a> {
    pub fn new(global: &'a Stack<'a>, args: &'a Args<'a>) -> Self {
        VarContext {
            global: global,
            parent: None,
            args: args
        }
    }

    pub fn nested_scope(&'a self, args: &'a Args<'a>) -> Self {
        VarContext {
            global: self.global,
            parent: Some(self),
            args: args
        }
    }

    pub fn template(&self) -> &'a Template {
        self.global.template()
    }

    pub fn global(&self) -> &'a Stack<'a> {
        self.global
    }

    pub fn env(&self) -> &'a Incrust {
        self.global.env()
    }

    pub fn get(&self, id: &str) -> Option<Arg<'a>> {
        self.args.get(id).map(Arg::from)
            .or_else(|| self.parent
                .and_then(|p| p.get(id))
                .or_else(|| self.global.env().top_context().get(id).map(Arg::from))
            )
    }
}
