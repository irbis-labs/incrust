
use ::abc::{Formatter, FormatResult, FormatError};
use ::incrust::Incrust;


#[derive(Debug)]
pub struct Escape;

impl Formatter for Escape {
    #[allow(unused_variables)]
    fn format(&self, value: &str, args: &[&str], env: &Incrust) -> FormatResult {
        use marksman_escape::Escape as F;
        String::from_utf8(F::new(value.bytes()).collect())
            .map_err(|err| FormatError::Process(format!("{:?}", err)))
    }
}


#[derive(Debug)]
pub struct Unescape;

impl Formatter for Unescape {
    #[allow(unused_variables)]
    fn format(&self, value: &str, args: &[&str], env: &Incrust) -> FormatResult {
        use marksman_escape::Unescape as F;
        String::from_utf8(F::new(value.bytes()).collect())
            .map_err(|err| FormatError::Process(format!("{:?}", err)))
    }
}

