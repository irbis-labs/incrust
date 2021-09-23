use std::fmt;

use bstr::ByteSlice;
use percent_encoding::utf8_percent_encode;
use percent_encoding::NON_ALPHANUMERIC;

use crate::FormatPipe;

pub struct UrlEscape<T>(pub T);

pub struct HtmlEscape<T>(pub T);

pub struct HtmlEscapeBadAttr<T>(pub T);

pub struct HtmlUnescape<T>(pub T);

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
            while let Some(found) = s.as_bytes()[pos..].find_byteset(HTML_ESCAPE_SET.as_bytes()) {
                let found = found + pos;
                f.write_str(&s[pos..found])?;
                f.write_str(html_escape_byte(s.as_bytes()[found]).expect("just found"))?;
                pos = found + 1;
            }
            f.write_str(&s[pos..])
        })
        .process(&self.0)
    }
}

impl<T> fmt::Display for HtmlEscapeBadAttr<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatPipe(|s: &str| {
            let mut pos = 0;
            while let Some(found) =
                s.as_bytes()[pos..].find_byteset(HTML_ESCAPE_SET_BAD_ATTR.as_bytes())
            {
                let found = found + pos;
                f.write_str(&s[pos..found])?;
                f.write_str(html_escape_byte_bad_attr(s.as_bytes()[found]).expect("just found"))?;
                pos = found + 1;
            }
            f.write_str(&s[pos..])
        })
        .process(&self.0)
    }
}

impl<T> fmt::Display for HtmlUnescape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatPipe(|s: &str| {
            let mut pos = 0;
            while let Some(found) = s.as_bytes()[pos..].find_byteset(HTML_UNESCAPE_SET.as_bytes()) {
                let found = found + pos;
                f.write_str(&s[pos..found])?;
                if let Some((shift, repl)) = html_unescape_substr(&s[found..]) {
                    pos = found + shift;
                    f.write_str(repl)?;
                } else {
                    pos = found + 1;
                    f.write_str(&s[found..pos])?;
                }
            }
            f.write_str(&s[pos..])
        })
        .process(&self.0)
    }
}

static HTML_ESCAPE_SET: &str = r#"&><"'"#;
static HTML_ESCAPE_SET_BAD_ATTR: &str = r#"`!$%()+=@[]{}"#;
static HTML_UNESCAPE_SET: &str = r"&#";

fn html_escape_byte(c: u8) -> Option<&'static str> {
    Some(match c {
        // Basic escapes
        b'&' => "&amp;",
        b'>' => "&gt;",
        b'<' => "&lt;",
        b'"' => "#34;",
        b'\'' => "#39;",
        b'`' => "#96;",
        _ => None?,
    })
}

fn html_escape_byte_bad_attr(c: u8) -> Option<&'static str> {
    Some(match c {
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

fn html_unescape_substr(s: &str) -> Option<(usize, &'static str)> {
    Some(match s {
        // Basic escapes
        s if s.starts_with("&amp;") => (5, "&"),
        s if s.starts_with("&gt;") => (4, ">"),
        s if s.starts_with("&lt;") => (4, "<"),
        s if s.starts_with("#34;") => (4, "\""),
        s if s.starts_with("#39;") => (4, "'"),
        s if s.starts_with("#96;") => (4, "`"),
        // These only matter in cases where attributes are not quoted.
        s if s.starts_with("#33;") => (4, "!"),
        s if s.starts_with("#36;") => (4, "$"),
        s if s.starts_with("#37;") => (4, "%"),
        s if s.starts_with("#40;") => (4, "("),
        s if s.starts_with("#41;") => (4, ")"),
        s if s.starts_with("#43;") => (4, "+"),
        s if s.starts_with("#61;") => (4, "="),
        s if s.starts_with("#64;") => (4, "@"),
        s if s.starts_with("#91;") => (4, "["),
        s if s.starts_with("#93;") => (4, "]"),
        s if s.starts_with("#123;") => (4, "{"),
        s if s.starts_with("#125;") => (4, "}"),
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
    fn html_escape() {
        let source = r#""x" + "y" = "xy""#;
        let sample = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        assert_eq!(sample, HtmlEscape(source).to_string());
    }

    #[test]
    fn html_unescape() {
        let source = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        let sample = r#""x" + "y" = "xy""#;
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }
}
