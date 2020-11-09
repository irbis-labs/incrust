use crate::template::render::RenderExtension;
use crate::{Args, Identifier, Incrust, Template, Value};

pub struct Context<'a> {
    env: &'a Incrust,
    args: Option<&'a Args<'a>>,
    stack_top: Option<&'a Context<'a>>,
    child: Option<&'a RenderExtension<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(env: &'a Incrust, args: Option<&'a Args<'a>>) -> Self {
        let stack_top = None;
        let child = None;
        Context {
            env,
            args,
            stack_top,
            child,
        }
    }

    pub fn var(&'a self, name: &Identifier) -> Option<&Value<'a>> {
        self.args
            .and_then(|args| args.get(name))
            .or_else(|| self.stack_top.and_then(|parent| parent.var(name)))
    }

    pub fn template(&'a self, name: &Identifier) -> Option<&'a Template> {
        self.env.get_template(name)
    }

    pub fn push(
        &'a self,
        args: Option<&'a Args<'a>>,
        child: Option<&'a RenderExtension<'a>>,
    ) -> Context<'a> {
        Context {
            env: self.env,
            args,
            stack_top: Some(self),
            child,
        }
    }

    pub fn extends(&'a self, child: &'a RenderExtension<'a>) -> Context<'a> {
        self.push(None, Some(child))
    }

    pub fn child(&'a self) -> Option<&'a RenderExtension<'a>> {
        self.child
    }
}
