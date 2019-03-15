use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
    digit,
};

use crate::container::pst::{self, ErrorKind::*};

pub fn number_literal(input: Slice) -> nom::IResult<Slice, pst::NumberLiteral, pst::ErrorKind> {
    // TODO allow underscore separator, e.g. 1_000_000
    let (next, part1) = complete!(input, digit)
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    // TODO hex, oct, and bin literals.
    let (output, part2) = recognize!(next,
        opt!(
            do_parse!(
                char!('.') >>
                digit >>
                (())
            )
        )
    )
        .map_err(|_| Failure(Code(input, Custom(IncorrectNumberLiteral))))?;

    let len = part1.len() + part2.len();
    let slice = &input[..len];
    Ok((output, pst::NumberLiteral(slice)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), pst::NumberLiteral(&sample[..]))),
            number_literal(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            number_literal(sample),
        );
    }

    fn _incorrect(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(IncorrectNumberLiteral)))),
            number_literal(sample),
        );
    }

    #[test]
    fn test() {
        good(r#"0"#);
        good(r#"42"#);
        good(r#"042"#);
        good(r#"0.0"#);
        good(r#"0.1"#);
        good(r#"1.0"#);

        // TODO this should change.
        // incorrect(r#"0xdeadbeef"#);
        // incorrect(r#"0o755"#);
        // incorrect(r#"0b00101010"#);
        // incorrect(r#"1_000"#);
        // incorrect(r#"1_000_"#);

        // NB this may (or may not) change.
        // incorrect(r#"1."#);
        not_recognized(r#".1"#);

        not_recognized(r#""#);
        not_recognized(r#" "#);
        not_recognized(r#"_0"#);
        not_recognized(r#"deadbeef"#);
        not_recognized(r#""string""#);
        not_recognized(r#""0""#);
        not_recognized(r#"'char'"#);
        not_recognized(r#"'0'"#);
        not_recognized(r#"plain text"#);
        not_recognized(r#"{# comment #}"#);
        not_recognized(r#"{% statement %}"#);
        not_recognized(r#"{{ expression }}"#);
    }
}
