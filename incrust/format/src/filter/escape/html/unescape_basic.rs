use std::fmt;

use bstr::ByteSlice;

use crate::FormatPipe;

static HTML_UNESCAPE_SET: &str = r"&#";

pub struct HtmlUnescape<T>(pub T);

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

fn html_unescape_substr(s: &str) -> Option<(usize, &'static str)> {
    Some(match s {
        // Basic escapes.
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
    fn html_unescape() {
        let source = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        let sample = r#""x" + "y" = "xy""#;
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }
}