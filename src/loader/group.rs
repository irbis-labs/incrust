use crate::abc::{Loader, LoadResult, LoadError};

pub type GroupLoader = Vec<Box<Loader>>;

impl Loader for Vec<Box<Loader>> {
    fn load(&self, name: &str) -> LoadResult {
        for loader in self {
            return match loader.load(name) {
                Err(LoadError::NotFound)    => continue,
                Err(err)                    => Err(err),
                Ok(res)                     => Ok(res),
            }
        }
        Err(LoadError::NotFound)
    }
}
