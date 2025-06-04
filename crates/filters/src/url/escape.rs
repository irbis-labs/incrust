use incrust_format::prelude::*;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use percent_encoding::utf8_percent_encode;

pub const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    // .add(b'\'')
    // .add(b'(')
    // .add(b')')
    .add(b'*')
    .add(b'+')
    // .add(b',')
    // .add(b'-')
    // .add(b'.')
    .add(b'/')
    .add(b':')
    // .add(b';')
    // .add(b'<')
    .add(b'=')
    // .add(b'>')
    .add(b'?')
    .add(b'@')
    // .add(b'[')
    .add(b'\\')
    // .add(b']')
    // .add(b'^')
    // .add(b'_')
    // .add(b'`')
    // .add(b'{')
    // .add(b'|')
    // .add(b'}')
    .add(b'~');

pub struct UrlEscape<T>(pub T);

impl<T> fmt::Display for UrlEscape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.format_with(|s: Option<&str>| {
            let Some(s) = s else { return Ok(()) };
            utf8_percent_encode(s, FRAGMENT).fmt(f)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_query_part() {
        let source = r##"#1. = "100%""##;
        let sample = r"%231.%20%3D%20%22100%25%22";
        assert_eq!(sample, UrlEscape(source).to_string());
    }
}
