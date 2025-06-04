use core::fmt;

use incrust_format::prelude::*;

const TAIL: &str = "]]>";

#[derive(Debug, Clone, Copy)]
pub struct XmlCData<T>(pub T);

impl<T: fmt::Display> fmt::Display for XmlCData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }
        write!(f, "<![CDATA[")?;

        let mut buf = PascalString::<2>::new();

        self.0.format_with(|string| {
            let Some(mut string) = string else {
                f.write_str(&buf)?;
                return Ok(());
            };

            match buf.as_str() {
                "]]" if string.starts_with(">") => {
                    f.write_str("]]]]><![CDATA[>")?;
                    string = &string[1..];
                }
                "]" if string.starts_with("]>") => {
                    f.write_str("]]]]><![CDATA[>")?;
                    string = &string[2..];
                }
                _ => (),
            }
            buf.clear();

            while let Some(tail_pos) = string.find(TAIL) {
                f.write_str(&string[..tail_pos])?;
                f.write_str("]]]]><![CDATA[>")?;
                string = &string[tail_pos + 3..];
            }

            let trim = match () {
                _ if string.ends_with("]]") => 2,
                _ if string.ends_with(']') => 1,
                _ => 0,
            };
            let (left, right) = string.split_at(string.len() - trim);
            f.write_str(left)?;
            debug_assert!(buf.is_empty());
            debug_assert!(buf.capacity() >= right.len());
            // Panic SAFETY: `buf` is always empty here and has enough capacity.
            buf.try_push_str(right).unwrap();

            Ok(())
        })?;
        write!(f, "]]>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        let text = "Hello, World!";
        let sample = "<![CDATA[Hello, World!]]>";
        let result = XmlCData(text).to_string();
        assert_eq!(sample, result);
    }

    #[test]
    fn test_tail() {
        let text = "]]>";
        let sample = "<![CDATA[]]]]><![CDATA[>]]>";
        let result = XmlCData(text).to_string();
        assert_eq!(sample, result);
    }

    #[test]
    fn test_tails_in_text() {
        let text = "<h1>Title</h1><p>Something about CData: <![CDATA[]]></p><p>some another text with tail ]]> and more text</p>";
        let sample = "<![CDATA[<h1>Title</h1><p>Something about CData: <![CDATA[]]]]><![CDATA[></p><p>some another text with tail ]]]]><![CDATA[> and more text</p>]]>";
        let result = XmlCData(text).to_string();
        assert_eq!(sample, result);
    }
}
