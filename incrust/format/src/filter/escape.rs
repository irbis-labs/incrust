use std::fmt;

use bstr::ByteSlice;
use percent_encoding::utf8_percent_encode;
use percent_encoding::NON_ALPHANUMERIC;

use crate::FormatPipe;

pub struct UrlEscape<T>(pub T);

pub struct HtmlEscape<T>(pub T);

impl<T> fmt::Display for UrlEscape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatPipe(|s: &str| utf8_percent_encode(s, NON_ALPHANUMERIC).fmt(f)).process(&self.0)
    }
}

impl<T> fmt::Display for HtmlEscape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatPipe(|s: &str| {
            let mut pos = 0;
            while let Some(found) = s.as_bytes()[pos..].find_byteset(REPL_SET.as_bytes()) {
                let found = found + pos;
                f.write_str(&s[pos..found])?;
                f.write_str(repl_byte(s.as_bytes()[found]).expect("just found"))?;
                pos = found + 1;
            }
            f.write_str(&s[pos..])
        })
        .process(&self.0)
    }
}

static REPL_SET: &str = r#"&><"'`!$%()+=@[]{}"#;

fn repl_byte(c: u8) -> Option<&'static str> {
    Some(match c {
        // Basic escapes
        b'&' => "&amp;",
        b'>' => "gt;",
        b'<' => "lt;",
        b'"' => "#34;",
        b'\'' => "#39;",
        b'`' => "#96;",
        // These only matter in cases where attributes are not quoted.
        b'!' => "#33;",
        b'$' => "#36;",
        b'%' => "#37;",
        b'(' => "#40;",
        b')' => "#41;",
        b'+' => "#43;",
        b'=' => "#61;",
        b'@' => "#64;",
        b'[' => "#91;",
        b']' => "#93;",
        b'{' => "#123;",
        b'}' => "#125;",
        _ => None?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url() {
        let source = r##"#1. "100%""##;
        let sample = r"%231%2E%20%22100%25%22";
        assert_eq!(sample, UrlEscape(source).to_string());
    }

    #[test]
    fn html() {
        let source = r#""x" + "y" = "xy""#;
        let sample = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        assert_eq!(sample, HtmlEscape(source).to_string());
    }
}
