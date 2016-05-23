use super::abc::*;


pub struct Context<'a> {
    parent_scope: Option<&'a Context<'a>>,
    local_scope: &'a Args<'a>,
}


impl <'a> Into<Context<'a>> for &'a Args<'a> { fn into(self) -> Context<'a> { Context::new(None, self) } }

impl <'a> Context<'a> {
    pub fn new(parent_scope: Option<&'a Context<'a>>, local_scope: &'a Args<'a>) -> Self {
        Context { parent_scope: parent_scope, local_scope: local_scope }
    }

    pub fn get(&self, id: &str) -> Option<&BType> {
        self.local_scope.get(id)
            .or_else(|| self.parent_scope.and_then(|scope| scope.get(id)))
    }
}

