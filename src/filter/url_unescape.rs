use std::borrow::Cow;

use abc::{Filter, FilterResult, FilterError};
use renderer::Writer;
use {Context, ex, Arg};


#[derive(Debug)]
pub struct UrlUnescape;

impl Filter for UrlUnescape {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a Context, value: Option<Arg<'a>>) -> FilterResult<Arg<'a>> {
        use url::percent_encoding::percent_decode;

        match value {
            None => Ok(None),
            Some(value) => {
                match value.try_as_string() {
                    None => {
                        let mut tmp = String::new();
                        value.render(&mut Writer(&mut tmp))?;
                        percent_decode(tmp.as_bytes())
                            .decode_utf8()
                            .map(Cow::into_owned)
                    },
                    Some(string) => {
                        percent_decode(string.as_bytes())
                            .decode_utf8()
                            .map(Cow::into_owned)
                    }
                }
                    .map(ex).map(Some)
                    .map_err(|err| FilterError::Process(format!("{:?}", err).into()))
            },
        }
    }
}
