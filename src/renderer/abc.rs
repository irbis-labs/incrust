use std::borrow::Cow;
use std::fmt;

use crate::abc::*;
use crate::{Arg, VarContext};


pub type EvalResult<T> = Result<Option<T>, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    Invoke(InvokeError),
    NoneArg,
    NotComposable,
    NotIndexable,
    NotMappable,
    UnexpectedIndexType,
    AttributeNotExists(Cow<'static, str>),
    IndexNotExists(usize),
    KeyNotExists(Cow<'static, str>),
    Input(Cow<'static, str>),
    Process(Cow<'static, str>),
}


#[derive(Debug)]
pub enum InvokeError {
    NotInvokable,
    WrongArgsNumber(usize, usize),
    WrongArgType(usize, ExpectedArgType),
}

#[derive(Debug)]
pub enum ExpectedArgType {
    String,
    Int,
    Real,
    Bool
}

impl From<InvokeError> for EvalError { fn from(err: InvokeError)  -> Self { EvalError::Invoke(err) } }


pub type FilterResult<T> = Result<Option<T>, FilterError>;

#[derive(Debug)]
pub enum FilterError {
    UnknownFormatter(Cow<'static, str>),
    Input(Cow<'static, str>),
    Process(Cow<'static, str>),
    Format(fmt::Error),
}

impl From<fmt::Error>  for FilterError { fn from(err: fmt::Error)  -> Self { FilterError::Format(err) } }


pub trait Filter: fmt::Debug + Send + Sync {
    fn filter<'s: 'a, 'a>(&'s self, context: &'a VarContext<'a>, value: Option<Arg<'a>>) -> FilterResult<Arg<'a>>;
}


pub type RenderResult<T> = Result<T, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    LoadTemplate(LoadError),
    ParseTemplate(TemplateParseError),
    VariableNotExists(Cow<'static, str>),
    EvalExpression(EvalError),
    Filter(FilterError),
    FunctionCallException(Cow<'static, str>),
    Format(fmt::Error),
}

impl From<LoadError>            for RenderError { fn from(err: LoadError)   -> Self { RenderError::LoadTemplate(err) } }
impl From<EvalError>            for RenderError { fn from(err: EvalError)   -> Self { RenderError::EvalExpression(err) } }
impl From<TemplateParseError>   for RenderError { fn from(err: TemplateParseError) -> Self { RenderError::ParseTemplate(err) } }
impl From<FilterError>          for RenderError { fn from(err: FilterError) -> Self { RenderError::Filter(err) } }
impl From<fmt::Error>           for RenderError { fn from(err: fmt::Error)  -> Self { RenderError::Format(err) } }


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
