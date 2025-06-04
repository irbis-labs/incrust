use incrust_format::prelude::*;

use crate::html::CC_LIST;
use crate::html::format;

const ESCAPE_TAG_LIST: [(char, &str); 6] = [
    ('"', "&quot;"),
    ('&', "&amp;"),
    ('\'', "&apos;"),
    ('<', "&lt;"),
    ('>', "&gt;"),
    ('`', "&grave;"),
];

const ESCAPE_TABLE: [&str; 128] = {
    let mut table = [""; 128];

    let mut i = 0;
    while i < CC_LIST.len() {
        let (ch, repl) = CC_LIST[i];
        table[ch as usize] = repl;
        i = i + 1;
    }

    let mut i = 0;
    while i < ESCAPE_TAG_LIST.len() {
        let (ch, repl) = ESCAPE_TAG_LIST[i];
        table[ch as usize] = repl;
        i = i + 1;
    }

    table
};

const ESCAPE_SET: u128 = {
    let mut set = 0;

    let mut i = 0;
    while i < ESCAPE_TABLE.len() {
        let bit = (!ESCAPE_TABLE[i].is_empty() as u128) << i;
        set = set | bit;
        i = i + 1;
    }

    set
};

#[inline]
fn html_escape_byte(c: u8) -> &'static str {
    match c {
        c if c < 32 => ESCAPE_TABLE[c as usize],
        // Basic escapes
        b'"' => "&quot;",
        b'&' => "&amp;",
        b'\'' => "&apos;",
        b'<' => "&lt;",
        b'>' => "&gt;",
        b'`' => "&grave;",
        _ => "",
    }
}

pub struct HtmlEscape<T>(pub T);

impl<T> fmt::Display for HtmlEscape<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.format_with(|s: Option<&str>| {
            let Some(string) = s else { return Ok(()) };
            format(f, string, ESCAPE_SET, html_escape_byte)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_list() {
        for (ch, repl) in CC_LIST.into_iter() {
            assert_eq!(repl, html_escape_byte(ch as u8));
        }

        for (ch, repl) in ESCAPE_TAG_LIST.into_iter() {
            assert_eq!(repl, html_escape_byte(ch as u8));
        }

        for ch in '\x20'..'\x7f' {
            if ESCAPE_TAG_LIST
                .into_iter()
                .position(|(c, _)| c == ch)
                .is_some()
            {
                continue;
            }
            let sample: PascalString<1> = ch.to_fmt();
            let result: PascalString<7> = HtmlEscape(sample).to_fmt();
            assert_eq!(sample, result);
        }
    }

    #[test]
    fn html_escape() {
        let source = r#""x" + "y" = "xy""#;
        let sample = r"&quot;x&quot; + &quot;y&quot; = &quot;xy&quot;";
        let result = dbg!(HtmlEscape(source).to_string());
        assert_eq!(sample, result);
    }
}
