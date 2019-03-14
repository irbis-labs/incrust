use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
    multispace1
};

use crate::{
    container::pst::{self, ErrorKind::*},
    parser::pst::is_end_of_token,
};

pub fn binary_operator(input: Slice) -> nom::IResult<Slice, pst::BinaryOperator, pst::ErrorKind> {
    let (output, op) = alt!(input,
        tuple!(tag!("not"), multispace1, tag!("in")) => { |_| pst::BinaryOperator::NotIn } |
        tag!("xor") => { |_| pst::BinaryOperator::Xor } |
        tag!("and") => { |_| pst::BinaryOperator::And } |
        tag!("==") => { |_| pst::BinaryOperator::Eq } |
        tag!("!=") => { |_| pst::BinaryOperator::NotEq } |
        tag!("<=") => { |_| pst::BinaryOperator::Lte } |
        tag!(">=") => { |_| pst::BinaryOperator::Gte } |
        tag!("in") => { |_| pst::BinaryOperator::In } |
        tag!("or") => { |_| pst::BinaryOperator::Or } |
        char!('<') => { |_| pst::BinaryOperator::Lt } |
        char!('>') => { |_| pst::BinaryOperator::Gt } |
        char!('+') => { |_| pst::BinaryOperator::Add } |
        char!('-') => { |_| pst::BinaryOperator::Sub } |
        char!('*') => { |_| pst::BinaryOperator::Mul } |
        char!('/') => { |_| pst::BinaryOperator::Div } |
        char!('%') => { |_| pst::BinaryOperator::Mod }
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

    fn good(sample: &str, op: pst::BinaryOperator) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), op)),
            binary_operator(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            binary_operator(sample),
        );
    }

    #[test]
    fn test() {
        good(r#"<"#, pst::BinaryOperator::Lt);
        good(r#">"#, pst::BinaryOperator::Gt);
        good(r#"+"#, pst::BinaryOperator::Add);
        good(r#"-"#, pst::BinaryOperator::Sub);
        good(r#"*"#, pst::BinaryOperator::Mul);
        good(r#"/"#, pst::BinaryOperator::Div);
        good(r#"%"#, pst::BinaryOperator::Mod);
        good(r#"=="#, pst::BinaryOperator::Eq);
        good(r#"!="#, pst::BinaryOperator::NotEq);
        good(r#"<="#, pst::BinaryOperator::Lte);
        good(r#">="#, pst::BinaryOperator::Gte);
        good(r#"in"#, pst::BinaryOperator::In);
        good(r#"or"#, pst::BinaryOperator::Or);
        good(r#"xor"#, pst::BinaryOperator::Xor);
        good(r#"and"#, pst::BinaryOperator::And);
        good(r#"not in"#, pst::BinaryOperator::NotIn);
        good(r#"not  in"#, pst::BinaryOperator::NotIn);

        not_recognized(r#""#);
        not_recognized(r#" "#);
        not_recognized(r#"_"#);
        not_recognized(r#"not"#);
        not_recognized(r#"in_"#);
        not_recognized(r#"or_"#);
        not_recognized(r#"and_"#);
        not_recognized(r#"xor_"#);
        not_recognized(r#"notin"#);
        not_recognized(r#"'-'"#);
        not_recognized(r#"not in_"#);
    }
}
