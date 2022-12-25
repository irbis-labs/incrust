use std::collections::HashMap;

use crate::{Args, Context, Identifier, Template};

#[derive(Default)]
pub struct Incrust {
    templates: HashMap<Identifier, Template>,
}

impl Incrust {
    pub fn new() -> Self {
        Incrust::default()
    }

    pub fn register_template(
        &mut self,
        name: impl Into<Identifier>,
        template: Template,
    ) -> Result<(), ()> {
        self.register_template_(name.into(), template)
    }

    fn register_template_(&mut self, name: Identifier, template: Template) -> Result<(), ()> {
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

    pub fn context<'a>(&'a self, args: impl Into<Option<&'a Args<'a>>>) -> Context<'a> {
        Context::new(self, args.into())
    }
}