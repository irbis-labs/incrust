use std::collections::hash_map::{HashMap};
use std::fs::File;
use std::path::{Path, PathBuf};

use ::abc::{Loader, LoadResult, LoadError};

pub type DictLoader = HashMap<String, String>;

impl Loader for HashMap<String, String> {
    fn load(&self, name: &str) -> LoadResult {
        match self.get(name) {
            Some(entry) => Ok(entry.to_owned()),
            None        => Err(LoadError::NotFound),
        }
    }
}
