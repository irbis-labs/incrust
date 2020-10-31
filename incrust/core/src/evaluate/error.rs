use thiserror::Error;

pub type EvalResult<T> = Result<T, EvalError>;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("not allowed operation")]
    NotAllowedOperation,
    #[error("unknown variable")]
    UnknownVariable,
}
