mod attribute;
mod escape;
mod escape_strict;
mod unescape_basic;

use core::fmt;

pub use self::escape::*;
pub use self::escape_strict::*;
pub use self::unescape_basic::*;
use crate::html::attribute::HtmlAttribute;

pub trait FilterHtml {
    fn html_attribute<N>(self, name: N) -> HtmlAttribute<N, Self>
    where
        Self: Sized + fmt::Display,
        N: fmt::Display;

    fn html_escape(self) -> HtmlEscape<Self>
    where
        Self: Sized + fmt::Display;

    fn html_escape_strict(self) -> HtmlEscapeStrict<Self>
    where
        Self: Sized + fmt::Display;

    fn html_unescape(self) -> HtmlUnescape<Self>
    where
        Self: Sized + fmt::Display;
}

impl<T: fmt::Display> FilterHtml for T {
    fn html_attribute<N>(self, name: N) -> HtmlAttribute<N, Self>
    where
        Self: Sized + fmt::Display,
        N: fmt::Display,
    {
        HtmlAttribute::new(name, self)
    }

    fn html_escape(self) -> HtmlEscape<Self>
    where
        Self: Sized + fmt::Display,
    {
        HtmlEscape(self)
    }

    fn html_escape_strict(self) -> HtmlEscapeStrict<Self>
    where
        Self: Sized + fmt::Display,
    {
        HtmlEscapeStrict(self)
    }

    fn html_unescape(self) -> HtmlUnescape<Self>
    where
        Self: Sized + fmt::Display,
    {
        HtmlUnescape(self)
    }
}

const CC_LIST: [(char, &str); 29] = [
    ('\0', ""),
    ('\x01', "&#1;"),
    ('\x02', "&#2;"),
    ('\x03', "&#3;"),
    ('\x04', "&#4;"),
    ('\x05', "&#5;"),
    ('\x06', "&#6;"),
    ('\x07', "&#7;"),
    ('\x08', "&#8;"),
    // ('\t', "&#9;"),
    // ('\n', "&#10;"),
    ('\x0b', "&#11;"),
    ('\x0c', "&#12;"),
    // ('\r', "&#13;"),
    ('\x0e', "&#14;"),
    ('\x0f', "&#15;"),
    ('\x10', "&#16;"),
    ('\x11', "&#17;"),
    ('\x12', "&#18;"),
    ('\x13', "&#19;"),
    ('\x14', "&#20;"),
    ('\x15', "&#21;"),
    ('\x16', "&#22;"),
    ('\x17', "&#23;"),
    ('\x18', "&#24;"),
    ('\x19', "&#25;"),
    ('\x1a', "&#26;"),
    ('\x1b', "&#27;"),
    ('\x1c', "&#28;"),
    ('\x1d', "&#29;"),
    ('\x1e', "&#30;"),
    ('\x1f', "&#31;"),
];

fn format(
    f: &mut fmt::Formatter<'_>,
    input: &str,
    set: u128,
    escape_byte: impl (FnOnce(u8) -> &'static str) + Copy,
) -> fmt::Result {
    let b = input.as_bytes();

    let mut pos = 0;

    while let Some(found_pos) = b[pos..]
        .iter()
        .copied()
        .position(|c| c < 128 && set & (1) << (c as usize) != 0)
    {
        let next_pos = found_pos + pos;

        f.write_str(&input[pos..next_pos])?;
        f.write_str(escape_byte(b[next_pos]))?;

        pos = next_pos + 1;
    }

    f.write_str(&input[pos..])
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static SIMPLE_CASES: &[(&str, &str)] = &[
        ("Hello, World!", "Hello, World!"),
        ("&amp;", "&amp;amp;"),
        ("\"Quotes\"", "&quot;Quotes&quot;"),
        ("'Single quotes'", "&apos;Single quotes&apos;"),
        ("<p>Content</p>", "&lt;p&gt;Content&lt;/p&gt;"),
        (
            "<script>alert('XSS');</script>",
            "&lt;script&gt;alert(&apos;XSS&apos;);&lt;/script&gt;",
        ),
        (
            "<a href=\"https://example.com\">Link</a>",
            "&lt;a href=&quot;https://example.com&quot;&gt;Link&lt;/a&gt;",
        ),
    ];

    #[test]
    fn it_escapes_and_unescapes_html_symbols() {
        fn test_case(expected: &str, sample: &str) {
            let escaped = sample.html_escape().to_string();
            assert_eq!(expected, escaped, "Failed to escape: {sample}");
            let unescaped = escaped.html_unescape().to_string();
            assert_eq!(sample, unescaped, "Failed to unescape: {sample}");
        }

        for &(sample, expected) in SIMPLE_CASES {
            test_case(expected, sample);
        }
    }

    pub static STRICT_CASES: &[(&str, &str)] = &[
        ("Hello, World!", "Hello, World&#33;"),
        ("&amp;", "&amp;amp;"),
        ("\"Quotes\"", "&quot;Quotes&quot;"),
        ("'Single quotes'", "&apos;Single quotes&apos;"),
        ("<p>Content</p>", "&lt;p&gt;Content&lt;/p&gt;"),
        (
            "<script>alert('XSS');</script>",
            "&lt;script&gt;alert&#40;&apos;XSS&apos;&#41;;&lt;/script&gt;",
        ),
        (
            "<a href=\"https://example.com\">Link</a>",
            "&lt;a href&#61;&quot;https://example.com&quot;&gt;Link&lt;/a&gt;",
        ),
    ];

    #[test]
    fn it_escapes_strictly_and_unescapes_html_symbols() {
        fn test_case(expected: &str, sample: &str) {
            let escaped = sample.html_escape_strict().to_string();
            assert_eq!(expected, escaped, "Failed to escape: {sample}");
            let unescaped = escaped.html_unescape().to_string();
            assert_eq!(sample, unescaped, "Failed to unescape: {sample}");
        }

        for &(sample, expected) in STRICT_CASES {
            test_case(expected, sample);
        }
    }
}
