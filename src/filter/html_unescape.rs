use abc::{Filter, FilterResult, FilterError};
use renderer::Writer;
use {VarContext, ex, Arg};


#[derive(Debug)]
pub struct Unescape;

impl Filter for Unescape {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a VarContext, value: Option<Arg<'a>>) -> FilterResult<Arg<'a>> {
        use marksman_escape::Unescape as F;

        match value {
            None => Ok(None),
            Some(value) => {
                match value.try_as_string() {
                    None => {
                        let mut tmp = String::new();
                        value.render(&mut Writer(&mut tmp))?;
                        String::from_utf8(F::new(tmp.bytes()).collect())
                    },
                    Some(string) => {
                        String::from_utf8(F::new(string.as_ref().bytes()).collect())
                    }
                }
                    .map(ex).map(Some)
                    .map_err(|err| FilterError::Process(format!("{:?}", err).into()))
            },
        }
    }
}

