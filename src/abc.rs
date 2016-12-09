use std::fmt;

use ::types::abc::{Type};
use ::types::context::{Context};
use ::incrust::Incrust;
use ::template::Template;


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


pub type EvalResult = Result<Option<Box<Type>>, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    NotInvocable,
    NoneArg,
    NotComposable,
    AttributeNotExists(String),
    Input(String),
    Process(String),
}


pub type FilterResult = Result<Option<String>, FilterError>;

#[derive(Debug)]
pub enum FilterError {
    UnknownFormatter(String),
    Input(String),
    Process(String),
}


pub trait Filter: fmt::Debug + Send + Sync {
    fn filter(&self, value: Option<String>, context: &Context) -> FilterResult;
}


pub type ParseResult = Result<Template, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Syntax(String),
}


pub type RenderResult<T> = Result<T, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    LoadTemplate(LoadError),
    ParseTemplate(ParseError),
    VariableNotExists(String),
    EvalExpression(EvalError),
    Filter(FilterError),
    FunctionCallException(String),
    Format(fmt::Error),
}

impl From<LoadError>   for RenderError { fn from(err: LoadError)   -> Self { RenderError::LoadTemplate(err) } }
impl From<EvalError>   for RenderError { fn from(err: EvalError)   -> Self { RenderError::EvalExpression(err) } }
impl From<ParseError>  for RenderError { fn from(err: ParseError)  -> Self { RenderError::ParseTemplate(err) } }
impl From<FilterError> for RenderError { fn from(err: FilterError) -> Self { RenderError::Filter(err) } }
impl From<fmt::Error>  for RenderError { fn from(err: fmt::Error)  -> Self { RenderError::Format(err) } }


//quick_error! {
//    #[derive(Debug)]
//    pub enum RenderError {
//        LoadTemplate(err: LoadError) {
//            from()
//        },
//        EvalExpression(err: EvalError) {
//            from()
//        },
//        ParseTemplate(err: ParseError) {
//            from()
//        },
//        Filter(err: FilterError) {
//            from()
//        },
//        Format(err: fmt::Error) {
//            from()
//        },
//        VariableNotExists(err: String) {
//
//        },
//        FunctionCallException(err: String) {
//
//        },
//    }
//}
