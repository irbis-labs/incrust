use std::fmt;

use bstr::ByteSlice;

use crate::FormatPipe;

static HTML_ESCAPE_SET: &str = r#"&><"'"#;

pub struct HtmlEscape<T>(pub T);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_escape() {
        let source = r#""x" + "y" = "xy""#;
        let sample = r"#34;x#34; + #34;y#34; = #34;xy#34;";
        assert_eq!(sample, HtmlEscape(source).to_string());
    }
}