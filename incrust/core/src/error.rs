use std::fmt;

use thiserror::Error;

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("unknown template")]
    UnknownTemplate,
    #[error("unknown variable")]
    UnknownVariable,
    #[error("not allowed operation")]
    NotAllowedOperation,
    #[error("boolean value expected")]
    BooleanExpected,
    #[error("iterator expected")]
    IteratorExpected,
}

pub type RenderResult<T> = Result<T, RenderError>;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("evaluation error")]
    Eval(#[from] EvalError),
    #[error("formatting error")]
    Format(#[from] fmt::Error),
}
