use std::fs::File;
use std::path::{Path, PathBuf};

use ::abc::{Loader, LoadResult, LoadError};

#[derive(Debug)]
pub struct NamespaceLoader {
    namespace: String,
    loader: Box<Loader>,
}

impl NamespaceLoader {
    pub fn new(namespace: &str, loader: Box<Loader>) -> Box<Self> {
        Box::new(NamespaceLoader { namespace: namespace.to_owned(), loader: loader })
    }
}

impl Loader for NamespaceLoader {
    fn load(&self, name: &str) -> LoadResult {
        match name.starts_with(self.namespace.as_str()) {
            true    => self.loader.load(&name[self.namespace.len()..]),
            false   => Err(LoadError::NotFound),
        }
    }
}
