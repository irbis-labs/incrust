use std::fmt::Debug;

use ::context::{Context};
use ::incrust::Incrust;
use ::template::Template;


pub type LoadResult = Result<String, LoadError>;

#[derive(Debug)]
pub enum LoadError {
    NotFound,
}


pub trait Loader: Debug {
    fn load(&self, name: &str) -> LoadResult;
}


pub type FilterResult = Result<Option<String>, FilterError>;

#[derive(Debug)]
pub enum FilterError {
    UnknownFormatter(String),
    Input(String),
    Process(String),
}


pub trait Filter: Debug {
    fn filter(&self, value: Option<String>, context: &Context, env: &Incrust) -> FilterResult;
}


pub type ParseResult = Result<Template, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Syntax(String),
}


pub type RenderResult = Result<String, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    LoadTemplate(LoadError),
    ParseTemplate(ParseError),
    VariableNotExists(String),
    Filter(FilterError),
    FunctionCallException(String),
}

impl From<LoadError> for RenderError {
    fn from(err: LoadError) -> Self { RenderError::LoadTemplate(err) }
}

impl From<ParseError> for RenderError {
    fn from(err: ParseError) -> Self { RenderError::ParseTemplate(err) }
}

impl From<FilterError> for RenderError {
    fn from(err: FilterError) -> Self { RenderError::Filter(err) }
}
