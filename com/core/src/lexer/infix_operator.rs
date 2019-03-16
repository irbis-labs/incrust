use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
    multispace1
};

use crate::{
    container::pst::{self, ErrorKind::*},
    lexer::is_end_of_identifier,
};

pub fn infix_operator(input: Slice) -> nom::IResult<Slice, pst::InfixOperator, pst::ErrorKind> {
    let (output, op) = alt!(input,
        tuple!(tag!("not"), multispace1, tag!("in")) => { |_| pst::InfixOperator::NotIn } |
        tag!("and") => { |_| pst::InfixOperator::And } |
        tag!("xor") => { |_| pst::InfixOperator::Xor } |
        tag!("==") => { |_| pst::InfixOperator::Eq } |
        tag!("!=") => { |_| pst::InfixOperator::NotEq } |
        tag!("<=") => { |_| pst::InfixOperator::Lte } |
        tag!(">=") => { |_| pst::InfixOperator::Gte } |
        tag!("in") => { |_| pst::InfixOperator::In } |
        tag!("or") => { |_| pst::InfixOperator::Or } |
        char!('<') => { |_| pst::InfixOperator::Lt } |
        char!('>') => { |_| pst::InfixOperator::Gt } |
        char!('+') => { |_| pst::InfixOperator::Add } |
        char!('-') => { |_| pst::InfixOperator::Sub } |
        char!('*') => { |_| pst::InfixOperator::Mul } |
        char!('/') => { |_| pst::InfixOperator::Div } |
        char!('%') => { |_| pst::InfixOperator::Mod }
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

    fn good(sample: &str, op: pst::InfixOperator) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), op)),
            infix_operator(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            infix_operator(sample),
        );
    }

    #[test]
    fn test() {
        good(r#"<"#, pst::InfixOperator::Lt);
        good(r#">"#, pst::InfixOperator::Gt);
        good(r#"+"#, pst::InfixOperator::Add);
        good(r#"-"#, pst::InfixOperator::Sub);
        good(r#"*"#, pst::InfixOperator::Mul);
        good(r#"/"#, pst::InfixOperator::Div);
        good(r#"%"#, pst::InfixOperator::Mod);
        good(r#"=="#, pst::InfixOperator::Eq);
        good(r#"!="#, pst::InfixOperator::NotEq);
        good(r#"<="#, pst::InfixOperator::Lte);
        good(r#">="#, pst::InfixOperator::Gte);
        good(r#"in"#, pst::InfixOperator::In);
        good(r#"or"#, pst::InfixOperator::Or);
        good(r#"xor"#, pst::InfixOperator::Xor);
        good(r#"and"#, pst::InfixOperator::And);
        good(r#"not in"#, pst::InfixOperator::NotIn);
        good(r#"not  in"#, pst::InfixOperator::NotIn);

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
