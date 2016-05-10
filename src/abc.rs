use std::fmt::Debug;
use std::collections::HashMap;

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


pub type FormatResult = Result<String, FormatError>;

#[derive(Debug)]
pub enum FormatError {
    UnknownFormatter(String),
    Input(String),
    Process(String),
}


pub trait Formatter: Debug {
    fn format(&self, value: &str, args: &[&str], env: &Incrust) -> FormatResult;
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
    Formatter(FormatError),
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

impl From<FormatError> for RenderError {
    fn from(err: FormatError) -> Self {
        RenderError::Formatter(err)
    }
}


pub type Args = HashMap<&'static str, &'static str>;
