use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice, multispace0};

use crate::{
    container::pst::{self, ErrorKind::*},
    lexer::*
};

pub fn operand(input: Slice) -> nom::IResult<Slice, pst::Operand, pst::ErrorKind> {
    alt!(input,
        tuple!(
            prefix_operator,
            fix_error!(pst::ErrorKind, multispace0),
            operand
        ) => { |(op_tr, _, op_nd)| pst::Operand::Prefix(op_tr, box op_nd) } |
        number_literal => { |res| pst::Operand::NumberLiteral(res) } |
        identifier => { |res| pst::Operand::Identifier(res) } |
        string_literal => { |res| pst::Operand::StringLiteral(res) } |
        char_literal => { |res| pst::Operand::CharLiteral(res) }
    )
        .map_err(|e| match e {
            Error(Code(output, Alt)) => Error(Code(output, Custom(NotRecognized))),
            e => e
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, check: impl Into<pst::Operand<'static>>) {
        let sample = Slice(sample.as_bytes());
        let check = check.into();
        assert_eq!(
            Ok((Slice(EMPTY), check)),
            operand(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            operand(sample),
        );
    }

    fn prefix(operator: pst::PrefixOperator, operand: impl Into<pst::Operand<'static>>) -> pst::Operand<'static> {
        pst::Operand::Prefix(operator, box operand.into())
    }

    #[test]
    fn test() {
        good(r#"42"#, pst::NumberLiteral(b"42"));
        good(r#"42.0"#, pst::NumberLiteral(b"42.0"));
        good(r#""string""#, pst::StringLiteral(b"string"));
        good(r#"'char'"#, pst::CharLiteral(b"char"));
        good(r#"identifier"#, pst::Identifier(b"identifier"));

        let check = prefix(pst::PrefixOperator::Minus, pst::NumberLiteral(b"42"));
        good(r#"-42"#, check.clone());
        let check = prefix(pst::PrefixOperator::Minus, check);
        good(r#"--42"#, check.clone());
        let check = prefix(pst::PrefixOperator::Minus, check);
        good(r#"---42"#, check);

        let check = prefix(pst::PrefixOperator::Minus, pst::NumberLiteral(b"42"));
        good(r#"- 42"#, check.clone());
        let check = prefix(pst::PrefixOperator::Minus, check);
        good(r#"-- 42"#, check.clone());
        let check = prefix(pst::PrefixOperator::Minus, check);
        good(r#"- -- 42"#, check);

        good(r#"-identifier"#, prefix(pst::PrefixOperator::Minus, pst::Identifier(b"identifier")));
        good(r#"- identifier"#, prefix(pst::PrefixOperator::Minus, pst::Identifier(b"identifier")));

        not_recognized(r#""#);
        not_recognized(r#"_42"#);
        not_recognized(r#"_identifier"#);
    }
}
