use std::fmt;

pub type LoadResult = Result<String, LoadError>;

#[derive(Debug, PartialEq)]
pub enum LoadError {
    BadName(String),
    IoError(String),
    NotFound,
}


pub trait Loader: fmt::Debug + Send + Sync {
    fn load(&self, name: &str) -> LoadResult;
}
