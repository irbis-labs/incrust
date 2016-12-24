use std::borrow::Cow;

use abc::{Filter, FilterResult, FilterError};
use renderer::Writer;
use {Context, ex, Arg};


#[derive(Debug)]
pub struct Escape;

impl Filter for Escape {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a Context, value: Option<Cow<'a, Arg>>) -> FilterResult<Cow<'a, Arg>> {
        use marksman_escape::Escape as F;

        match value {
            None => Ok(None),
            Some(value) => {
                match value.as_ref().try_as_string() {
                    None => {
                        let mut tmp = String::new();
                        value.render(&mut Writer(&mut tmp))?;
                        String::from_utf8(F::new(tmp.bytes()).collect())
                    },
                    Some(string) => {
                        String::from_utf8(F::new(string.as_ref().bytes()).collect())
                    }
                }
                    .map(|v| Some(Cow::Owned(ex(v))))
                    .map_err(|err| FilterError::Process(format!("{:?}", err)))
            },
        }
    }
}


#[derive(Debug)]
pub struct Unescape;

impl Filter for Unescape {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a Context, value: Option<Cow<'a, Arg>>) -> FilterResult<Cow<'a, Arg>> {
        use marksman_escape::Unescape as F;

        match value {
            None => Ok(None),
            Some(value) => {
                match value.as_ref().try_as_string() {
                    None => {
                        let mut tmp = String::new();
                        value.render(&mut Writer(&mut tmp))?;
                        String::from_utf8(F::new(tmp.bytes()).collect())
                    },
                    Some(string) => {
                        String::from_utf8(F::new(string.as_ref().bytes()).collect())
                    }
                }
                    .map(|v| Some(Cow::Owned(ex(v))))
                    .map_err(|err| FilterError::Process(format!("{:?}", err)))
            },
        }
    }
}

