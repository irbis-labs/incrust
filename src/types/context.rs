use super::abc::*;
use incrust::Incrust;
use container::Template;


pub type TemplateStack<'a> = Vec<&'a Template>;


pub struct GlobalContext<'a> {
    env: &'a Incrust,
    template: &'a Template,
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
    pub fn new(template: &'a Template, env: &'a Incrust) -> Self {
        GlobalContext {
            env: env,
            template: template,
        }
    }

    pub fn nest(&'a self, args: &'a Args<'a>) -> Context<'a> {
        Context::new(self, args)
    }

    pub fn template(&self) -> &'a Template {
        self.template
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

    pub fn nest(&'a self, args: &'a Args<'a>) -> Self {
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
