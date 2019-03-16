use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
};

use crate::{
    container::pst::{self, ErrorKind::*},
    lexer::is_end_of_identifier,
};

pub fn prefix_operator(input: Slice) -> nom::IResult<Slice, pst::PrefixOperator, pst::ErrorKind> {
    let (output, op) = alt!(input,
        char!('-') => { |_| pst::PrefixOperator::Minus } |
        tag!("not") => { |_| pst::PrefixOperator::Not }
    )
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    if op.is_keyword() && !is_end_of_identifier(output) {
        Err(Error(Code(input, Custom(NotRecognized))))
    } else {
        Ok((output, op))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, op: pst::PrefixOperator) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), op)),
            prefix_operator(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            prefix_operator(sample),
        );
    }

    #[test]
    fn test() {
        good(r#"-"#, pst::PrefixOperator::Minus);
        good(r#"not"#, pst::PrefixOperator::Not);

        not_recognized(r#""#);
        not_recognized(r#" "#);
        not_recognized(r#"+"#);
        not_recognized(r#"_"#);
        not_recognized(r#"'-'"#);
        not_recognized(r#"not_"#);
    }
}
