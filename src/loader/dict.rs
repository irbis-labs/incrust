use std::borrow::Cow;
use std::collections::hash_map::{HashMap};
use std::hash::BuildHasher;

use ::abc::{Loader, LoadResult, LoadError};

pub type DictLoader = HashMap<Cow<'static, str>, Cow<'static, str>>;

impl<S> Loader for HashMap<Cow<'static, str>, Cow<'static, str>, S>
where
    S: BuildHasher + Sync + Send,
{
    fn load(&self, name: &str) -> LoadResult {
        match self.get(name) {
            Some(entry) => Ok(entry.to_owned()),
            None        => Err(LoadError::NotFound),
        }
    }
}
