use super::abc::*;
use ::incrust::Incrust;


pub enum ParentScope<'a> {
    Env(&'a Incrust),
    Context(&'a Context<'a>),
}

impl <'a> ParentScope<'a> {
    pub fn get(&self, id: &str) -> Option<&BType> {
        match *self {
            ParentScope::Env(ref env) => env.top_context().get(id),
            ParentScope::Context(ref context) => context.get(id)
        }
    }

    pub fn env(&self) -> &'a Incrust {
        match *self {
            ParentScope::Env(ref env) => env,
            ParentScope::Context(ref context) => context.env()
        }
    }
}

impl <'a> From<&'a Incrust> for ParentScope<'a> {
    fn from(x: &'a Incrust) -> Self {
        ParentScope::Env(x)
    }
}


pub struct Context<'a> {
    parent_scope: ParentScope<'a>,
    local_scope: &'a Args<'a>,
}


impl <'a> Context<'a> {
    pub fn new<PS: Into<ParentScope<'a>>>(parent_scope: PS, local_scope: &'a Args<'a>) -> Self {
        Context { parent_scope: parent_scope.into(), local_scope: local_scope }
    }

    pub fn nest(&'a self, local_scope: &'a Args<'a>) -> Self {
        Context { parent_scope: ParentScope::Context(self), local_scope: local_scope }
    }

    pub fn env(&self) -> &'a Incrust {
        self.parent_scope.env()
    }

    pub fn get(&self, id: &str) -> Option<&BType> {
        self.local_scope.get(id)
            .or_else(|| self.parent_scope.get(id))
    }
}
