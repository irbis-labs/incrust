use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{self, ErrorKind::*};

pub fn plaintext(input: Slice) -> nom::IResult<Slice, pst::PlainText, pst::ErrorKind> {
    if input.is_empty() {
        Err(Error(Code(input, Custom(NotRecognized))))?;
    }
    let mut next = input;
    let mut len = 0;
    loop {
        if let Ok((nxt, slice)) = is_not!(next, b"{") {
            next = nxt;
            len += slice.len();
        }
        if next.is_empty() || end_of_plaintext(input) {
            break;
        }
        next = Slice(&next[1..]);
        len += 1;
    }
    let slice = &input[..len];
    if slice.is_empty() {
        Err(Error(Code(input, Custom(NotRecognized))))?;
    }
    Ok((next, pst::PlainText(slice)))
}

fn end_of_plaintext(input: Slice) -> bool {
    pair!(input,
        char!('{'),
        alt!(
            char!('{') |
            char!('%') |
            char!('#')
        )
    )
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), pst::PlainText(&sample[..]))),
            plaintext(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            plaintext(sample),
        );
    }

    #[test]
    fn test() {
        good(" ");
        good("\n");
        good("\r");
        good("\t");
        good("\\");
        good(r#"\n"#);
        good(r#"\r"#);
        good(r#"\t"#);
        good(r#"\\"#);
        good("Just text");
        good(r#""Could be a string literal""#);
        good(r#"'Could be a char literal'"#);
        good(r#"42"#);
        good(r#"3.1415"#);
        good(r#"-1"#);
        good("{ # Almost comment # }");
        good("{ { Almost expression } }");
        good("{ % Almost statement % }");
        good("{! Something special !}");
        good("{");
        good("{}");
        good("{}}");
        good("}}");
        good("%}");
        good("#}");

        not_recognized("");
    }
}
