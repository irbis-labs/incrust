use crate::{Args, Incrust, Identifier, Value, Template};

pub struct Context<'a> {
    env: &'a Incrust,
    args: &'a Args<'a>,
}

impl<'a> Context<'a> {
    pub fn new(env: &'a Incrust, args: &'a Args<'a>) -> Self {
        Context { env, args }
    }

    pub fn var(&'a self, name: &Identifier) -> Option<&Value<'a>> {
        self.args.get(name)
    }

    pub fn template(&'a self, name: &Identifier) -> Option<&Template> {
        self.env.get_template(name)
    }
}
