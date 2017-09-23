use std::borrow::Cow;

use ::abc::{Loader, LoadResult, LoadError};


#[derive(Debug)]
pub struct NamespaceLoader {
    namespace: Cow<'static, str>,
    loader: Box<Loader>,
}

impl NamespaceLoader {
    pub fn new(namespace: &str, loader: Box<Loader>) -> Box<Self> {
        let namespace = namespace.to_owned().into();
        Box::new(NamespaceLoader { namespace, loader })
    }
}

impl Loader for NamespaceLoader {
    fn load(&self, name: &str) -> LoadResult {
        match name.starts_with(self.namespace.as_ref()) {
            true    => self.loader.load(&name[self.namespace.len()..]),
            false   => Err(LoadError::NotFound),
        }
    }
}
