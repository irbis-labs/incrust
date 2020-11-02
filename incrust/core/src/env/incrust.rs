use std::collections::HashMap;

use crate::{Identifier, Template, Args, Context};

#[derive(Default)]
pub struct Incrust {
    templates: HashMap<Identifier, Template>,
}

impl Incrust {
    pub fn new() -> Self {
        Incrust::default()
    }

    pub fn register_template(&mut self, name: Identifier, template: Template) -> Result<(), ()> {
        use std::collections::hash_map::Entry;

        match self.templates.entry(name) {
            Entry::Occupied(_) => Err(())?,
            Entry::Vacant(e) => e.insert(template),
        };
        Ok(())
    }

    pub fn get_template(&self, name: &Identifier) -> Option<&Template> {
        self.templates.get(name)
    }

    pub fn context<'a>(&'a self, args: &'a Args<'a>) -> Context<'a> {
        Context::new(self, args)
    }
}
