use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
};

use crate::container::pst::{self, ErrorKind::*};

pub fn string_literal(input: Slice) -> nom::IResult<Slice, pst::StringLiteral, pst::ErrorKind> {
    let (next, _) = complete!(input, char!('"'))
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    let (next, string) = recognize!(next, check_string)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStringLiteral))))?;

    let (output, _) = char!(next, '"')
        .map_err(|_| Failure(Code(input, Custom(UnclosedStringLiteral))))?;

    Ok((output, pst::StringLiteral(&string[..])))
}

fn check_string(input: Slice) -> nom::IResult<Slice, (), pst::ErrorKind> {
    // TODO number encoded symbols, e.g. \x00
    // FIXME check utf8
    let (output, _) = fold_many0!(input,
        alt!(
            is_not!(br#""\"#) |
            alt!(
                tag!(r#"\\"#) |
                tag!(r#"\""#) |
                tag!(r#"\n"#) |
                tag!(r#"\r"#) |
                tag!(r#"\t"#) |
                tag!(r#"\"#)
            )
        ),
        (),
        |_a, _i| ()
    )
        .map_err(|_| Failure(Code(input, Custom(IncorrectStringLiteral))))?;

    Ok((output, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), pst::StringLiteral(&sample[1..sample.len() - 1]))),
            string_literal(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            string_literal(sample),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedStringLiteral)))),
            string_literal(sample),
        );
    }

    #[test]
    fn test() {
        good(r#""""#);
        good(r#"" ""#);
        good(r#""string""#);
        good(r#""a\nb""#);
        good(r#""0""#);
        good(r#""0.0""#);
        good(r#""a\rb""#);
        good(r#""a\tb""#);
        good(r#""a\\b""#);
        good(r#""a\"b""#);
        good(r#""a'b""#);
        good(r#""\\""#);
        good(r#""\ \\""#);
        good(r#""{{ expression }}""#);
        good(r#""{% statement %}""#);
        good(r#""{# comment #}""#);

        not_recognized(r#""#);
        not_recognized(r#"''"#);
        not_recognized(r#"plain text"#);
        not_recognized(r#"{{ expression }}"#);
        not_recognized(r#"\""#);
        not_recognized(r#"\"""#);
        not_recognized(r#"\"\""#);

        unclosed(r#"""#);
        unclosed(r#""\""#);
        unclosed(r#""\\\""#);
        unclosed(r#""with text"#);
        unclosed(r#""with text\""#);
    }
}
