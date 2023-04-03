use bstr::ByteSlice;

use crate::util::prelude::*;
use crate::util::StrBuffer;

static HTML_UNESCAPE_BEGIN_SET: &str = r"&#";
static HTML_UNESCAPE_END: &str = r";";
const MAX_LEN: usize = 5;

pub struct HtmlUnescape<T>(pub T);

impl<T> fmt::Display for HtmlUnescape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = StrBuffer::<MAX_LEN>::new();
        RevFmt::new(|s: &str| {
            // eprintln!("=>: {s:?};  {:?}", buf.as_str());
            if s.is_empty() {
                if !buf.is_empty() {
                    f.write_str(buf.as_str())?;
                    buf.clear();
                }
                return Ok(());
            }

            let mut pos = 0;
            if !buf.is_empty() {
                if let Some(end) = s.find(HTML_UNESCAPE_END) {
                    let end = end + 1;
                    // eprintln!("A0: {end:?};  {:?}", &s[..end]);
                    while buf.push_str(&s[..end]).is_err() {
                        // It is expected to be Some if buf is not empty.
                        let first = buf.first().unwrap();
                        f.write_str(first)?;
                        // eprintln!("A1: {first:?}");
                        let _ = buf.strip_first();
                        // eprintln!("A2: {:?}", buf.as_str());
                        if buf.is_empty() {
                            break;
                        }
                    }
                    // eprintln!("A3: {:?}", buf.as_str());
                    if buf.is_empty() {
                        pos = end;
                    } else {
                        if let Some((shift, repl)) = html_unescape_substr(buf.as_str()) {
                            debug_assert_eq!(shift, buf.len());
                            f.write_str(repl)?;
                        } else {
                            f.write_str(buf.as_str())?;
                        }
                        buf.clear();
                        pos = end;
                    }
                } else {
                    while buf.push_str(s).is_err() {
                        // It is expected to be Some if buf is not empty.
                        let first = buf.first().unwrap();
                        f.write_str(first)?;
                        // eprintln!("B1: {first:?}");
                        let _ = buf.strip_first();
                        // eprintln!("B2: {:?}", buf.as_str());
                        if buf.is_empty() {
                            break;
                        }
                    }
                    if !buf.is_empty() {
                        // eprintln!("C1: {:?}", buf.as_str());
                        if let Some((shift, repl)) = html_unescape_substr(buf.as_str()) {
                            debug_assert_eq!(shift, buf.len());
                            f.write_str(repl)?;
                            buf.clear();
                        }
                        return Ok(());
                    }
                };
            }

            while let Some(found) =
                s.as_bytes()[pos..].find_byteset(HTML_UNESCAPE_BEGIN_SET.as_bytes())
            {
                let begin = found + pos;
                f.write_str(&s[pos..begin])?;
                if let Some(end) = s[begin..].find(HTML_UNESCAPE_END) {
                    if end > begin + MAX_LEN {
                        pos = begin + 1;
                        f.write_str(&s[begin..pos])?;
                    } else {
                        if let Some((shift, repl)) = html_unescape_substr(&s[begin..]) {
                            pos = begin + shift;
                            f.write_str(repl)?;
                        } else {
                            pos = begin + 1;
                            f.write_str(&s[begin..pos])?;
                        }
                    }
                } else {
                    if buf.push_str(&s[begin..]).is_ok() {
                        return Ok(());
                    } else {
                        pos = begin + 1;
                        f.write_str(&s[begin..pos])?;
                    }
                }
            }
            f.write_str(&s[pos..])?;
            Ok(())
        })
        .format(&self.0)
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
    use crate::Join;

    #[test]
    fn test_html_unescape_example() {
        let source = r"#34;x#34; #43; #34;y#34; #61; #34;xy#34;";
        let sample = r#""x" + "y" = "xy""#;
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }

    #[test]
    fn test_html_unescape() {
        let source = r"&amp;";
        let sample = r"&";
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }

    #[test]
    fn test_html_unescape_splitted() {
        let source = r"&amp;gt;".split("").filter(|c| !c.is_empty());
        let sample = r"&gt;";
        assert_eq!(sample, HtmlUnescape(Join("", source)).to_string());
    }
}
