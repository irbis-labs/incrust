use std::borrow::Cow;
use std::fmt;

pub type LoadResult = Result<Cow<'static, str>, LoadError>;

#[derive(Debug, PartialEq)]
pub enum LoadError {
    BadName(Cow<'static, str>),
    IoError(Cow<'static, str>),
    NotFound,
}


pub trait Loader: fmt::Debug + Send + Sync {
    fn load(&self, name: &str) -> LoadResult;
}
