use std::fmt;

use percent_encoding::utf8_percent_encode;
use percent_encoding::NON_ALPHANUMERIC;

use crate::FormatPipe;

pub struct UrlEscape<T>(pub T);

impl<T> fmt::Display for UrlEscape<T>
    where
        T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatPipe(|s: &str| utf8_percent_encode(s, NON_ALPHANUMERIC).fmt(f)).process(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_query_part() {
        let source = r##"#1. = "100%""##;
        let sample = r"%231%2E%20%3D%20%22100%25%22";
        assert_eq!(sample, UrlEscape(source).to_string());
    }
}
