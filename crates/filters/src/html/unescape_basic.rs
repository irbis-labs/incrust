use incrust_format::prelude::*;

const ENTITY_BEGIN: u8 = b'&';
const ENTITY_END: u8 = b';';
const MAX_LEN: usize = 7;

pub struct HtmlUnescape<T>(pub T);

impl<T> fmt::Display for HtmlUnescape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = StrBuffer::<MAX_LEN>::new();

        self.0.format_with(|s: Option<&str>| {
            // println!("=>: {s:?};  {:?}", buf.as_str());

            let Some(string) = s else {
                if !buf.is_empty() {
                    f.write_str(buf.as_str())?;
                    buf.clear();
                }
                return Ok(());
            };

            let bytes = string.as_bytes();

            let mut pos = 0;
            if !buf.is_empty() {
                if let Some(end) = bytes.iter().copied().position(|c| c == ENTITY_END) {
                    let end = end + 1;
                    while buf.push_str(&string[..end]).is_err() {
                        // It is expected to be Some if buf is not empty.
                        let first = buf.first().unwrap();
                        f.write_str(first)?;
                        let _ = buf.strip_first();
                        if buf.is_empty() {
                            break;
                        }
                    }
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
                    while buf.push_str(string).is_err() {
                        // It is expected to be Some if buf is not empty.
                        let first = buf.first().unwrap();
                        f.write_str(first)?;
                        let _ = buf.strip_first();
                        if buf.is_empty() {
                            break;
                        }
                    }
                    if !buf.is_empty() {
                        if let Some((shift, repl)) = html_unescape_substr(buf.as_str()) {
                            debug_assert_eq!(shift, buf.len());
                            f.write_str(repl)?;
                            buf.clear();
                        }
                        return Ok(());
                    }
                };
            }

            while let Some(found) = bytes[pos..].iter().copied().position(|c| c == ENTITY_BEGIN) {
                let begin = found + pos;
                f.write_str(&string[pos..begin])?;
                if let Some(end) = bytes[begin..].iter().copied().position(|c| c == ENTITY_END) {
                    if end > begin + MAX_LEN {
                        pos = begin + 1;
                        f.write_str(&string[begin..pos])?;
                    } else {
                        if let Some((shift, repl)) = html_unescape_substr(&string[begin..]) {
                            pos = begin + shift;
                            f.write_str(repl)?;
                        } else {
                            pos = begin + 1;
                            f.write_str(&string[begin..pos])?;
                        }
                    }
                } else {
                    if buf.push_str(&string[begin..]).is_ok() {
                        return Ok(());
                    } else {
                        pos = begin + 1;
                        f.write_str(&string[begin..pos])?;
                    }
                }
            }
            f.write_str(&string[pos..])?;
            Ok(())
        })
    }
}

fn html_unescape_substr(s: &str) -> Option<(usize, &'static str)> {
    Some(match s {
        s if s.starts_with("&amp;") => (5, "&"),
        s if s.starts_with("&gt;") => (4, ">"),
        s if s.starts_with("&lt;") => (4, "<"),
        s if s.starts_with("&quot;") => (6, "\""),
        s if s.starts_with("&apos;") => (6, "'"),
        s if s.starts_with("&grave;") => (7, "`"),

        s if s.starts_with("&#33;") => (5, "!"),
        s if s.starts_with("&#34;") => (5, "\""),
        s if s.starts_with("&#36;") => (5, "$"),
        s if s.starts_with("&#37;") => (5, "%"),
        s if s.starts_with("&#38;") => (5, "&"),
        s if s.starts_with("&#39;") => (5, "'"),
        s if s.starts_with("&#40;") => (5, "("),
        s if s.starts_with("&#41;") => (5, ")"),
        s if s.starts_with("&#43;") => (5, "+"),
        s if s.starts_with("&#60;") => (5, "<"),
        s if s.starts_with("&#61;") => (5, "="),
        s if s.starts_with("&#62;") => (5, ">"),
        s if s.starts_with("&#64;") => (5, "@"),
        s if s.starts_with("&#91;") => (5, "["),
        s if s.starts_with("&#93;") => (5, "]"),
        s if s.starts_with("&#96;") => (5, "`"),
        s if s.starts_with("&#123;") => (6, "{"),
        s if s.starts_with("&#125;") => (6, "}"),
        _ => None?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::JoinIter;

    #[test]
    fn test_html_unescape_example() {
        let source = r"&quot;x&quot; &#43; &quot;y&quot; &#61; &quot;xy&quot;";
        let sample = r#""x" + "y" = "xy""#;
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }

    #[test]
    fn test_html_unescape() {
        let source = r"&amp;";
        let sample = r"&";
        assert_eq!(sample, HtmlUnescape(source).to_string());
    }

    // #[test]
    // fn test_html_unescape_splitted() {
    //     let source = r"&amp;gt;".split("").filter(|c| !c.is_empty());
    //     let sample = r"&gt;";
    //     assert_eq!(sample, HtmlUnescape(JoinIter("", source)).to_string());
    // }
}
