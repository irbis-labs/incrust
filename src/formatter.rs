
use ::abc::{Filter, FilterResult, FilterError};
use ::incrust::{Incrust, Context};


#[derive(Debug)]
pub struct Escape;

impl Filter for Escape {
    #[allow(unused_variables)]
    fn filter(&self, value: Option<String>, context: &Context, env: &Incrust) -> FilterResult {
        use marksman_escape::Escape as F;
        match value {
            Some(string) => {
                String::from_utf8(F::new(string.bytes()).collect())
                    .map(Some)
                    .map_err(|err| FilterError::Process(format!("{:?}", err)))
            },
            None => Ok(None)
        }
    }
}


#[derive(Debug)]
pub struct Unescape;

impl Filter for Unescape {
    #[allow(unused_variables)]
    fn filter(&self, value: Option<String>, context: &Context, env: &Incrust) -> FilterResult {
        use marksman_escape::Unescape as F;
        match value {
            Some(string) => {
                String::from_utf8(F::new(string.bytes()).collect())
                    .map(Some)
                    .map_err(|err| FilterError::Process(format!("{:?}", err)))
            },
            None => Ok(None)
        }
    }
}

