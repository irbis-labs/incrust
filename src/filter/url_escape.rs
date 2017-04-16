use abc::{Filter, FilterResult};
use renderer::Writer;
use {Context, ex, Arg};


#[derive(Debug)]
pub struct UrlEscape;

impl Filter for UrlEscape {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a Context, value: Option<Arg<'a>>) -> FilterResult<Arg<'a>> {
        use url::percent_encoding::utf8_percent_encode;
        use url::percent_encoding::PATH_SEGMENT_ENCODE_SET;

        match value {
            None => Ok(None),
            Some(value) => {
                let value = match value.try_as_string() {
                    None => {
                        let mut tmp = String::new();
                        value.render(&mut Writer(&mut tmp))?;
                        utf8_percent_encode(&tmp, PATH_SEGMENT_ENCODE_SET)
                            .collect::<String>()
                    },
                    Some(string) => {
                        utf8_percent_encode(&string, PATH_SEGMENT_ENCODE_SET)
                            .collect::<String>()
                    }
                };
                Ok(Some(ex(value)))
            },
        }
    }
}
