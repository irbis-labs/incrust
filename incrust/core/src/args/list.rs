use std::collections::HashMap;

use crate::args::Identifier;
use crate::value::Value;

#[derive(Default)]
pub struct Args<'a> {
    map: HashMap<Identifier, Value<'a>>,
}

impl<'a> Args<'a> {
    #[inline(always)]
    pub fn new() -> Self {
        Args::default()
    }

    #[inline(always)]
    pub fn insert(&mut self, name: impl Into<Identifier>, value: impl Into<Value<'a>>) {
        let name = name.into();
        let value = value.into();
        self.map.insert(name, value);
    }

    #[inline(always)]
    pub fn get(&self, name: &Identifier) -> Option<&Value<'a>> {
        self.map.get(name)
    }
}
