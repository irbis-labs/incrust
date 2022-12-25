use std::fmt;

use bstr::ByteSlice;

use crate::FormatPipe;

static HTML_ESCAPE_SET_BAD_ATTR: &str = r#"&><"'!$%()+=@[]{}"#;

pub struct HtmlEscapeBadAttr<T>(pub T);

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

fn html_escape_byte_bad_attr(c: u8) -> Option<&'static str> {
    Some(match c {
        // Basic escapes.
        b'&' => "&amp;",
        b'>' => "&gt;",
        b'<' => "&lt;",
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
    fn html_escape_bad_attr() {
        let source = r#""x" + "y" = "xy""#;
        let sample = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        assert_eq!(sample, HtmlEscapeBadAttr(source).to_string());
    }
}