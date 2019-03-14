use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
};

use crate::{
    container::pst::{self, ErrorKind::*},
    parser::pst::is_end_of_token,
};

pub fn unary_operator(input: Slice) -> nom::IResult<Slice, pst::UnaryOperator, pst::ErrorKind> {
    let (output, op) = alt!(input,
        char!('-') => { |_| pst::UnaryOperator::Minus } |
        tag!("not") => { |_| pst::UnaryOperator::Not }
    )
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    if is_end_of_token(output) {
        Ok((output, op))
    } else {
        Err(Error(Code(input, Custom(NotRecognized))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, op: pst::UnaryOperator) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), op)),
            unary_operator(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            unary_operator(sample),
        );
    }

    #[test]
    fn test() {
        good(r#"-"#, pst::UnaryOperator::Minus);
        good(r#"not"#, pst::UnaryOperator::Not);

        not_recognized(r#""#);
        not_recognized(r#" "#);
        not_recognized(r#"+"#);
        not_recognized(r#"_"#);
        not_recognized(r#"'-'"#);
        not_recognized(r#"not_"#);
    }
}
