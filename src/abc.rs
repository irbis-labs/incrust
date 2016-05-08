use std::fmt::Debug;
use super::parser;


pub type LoadResult = Result<String, LoadError>;

#[derive(Debug)]
pub enum LoadError {
    NotFound,
}


pub trait Loader: Debug {
    fn load(&self, name: &str) -> LoadResult;
}


pub type ParseResult = Result<parser::Template, ParseError>;

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
    FunctionCallException(String),
}

impl From<LoadError> for RenderError {
    fn from(err: LoadError) -> Self {
        RenderError::LoadTemplate(err)
    }
}

impl From<ParseError> for RenderError {
    fn from(err: ParseError) -> Self {
        RenderError::ParseTemplate(err)
    }
}

