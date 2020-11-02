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
}
