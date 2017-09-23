use std::borrow::Cow;
use std::collections::hash_map::{HashMap};

use ::abc::{Loader, LoadResult, LoadError};

pub type DictLoader = HashMap<Cow<'static, str>, Cow<'static, str>>;

impl Loader for HashMap<Cow<'static, str>, Cow<'static, str>> {
    fn load(&self, name: &str) -> LoadResult {
        match self.get(name) {
            Some(entry) => Ok(entry.to_owned()),
            None        => Err(LoadError::NotFound),
        }
    }
}
