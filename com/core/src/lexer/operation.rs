use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice, multispace0};

use crate::{
    container::pst::{*, ErrorKind::*},
    lexer::*
};

pub fn operation_fold0<'i>(input: Slice<'i>) -> nom::IResult<Slice<'i>, Operations<'i>, ErrorKind> {
    let (next, left) = operand(input)?;
    fold_many0!(next,
        operation,
        Operations(left, Vec::new()),
        move |mut acc: Operations<'i>, (op, right)| {
            acc.1.push((op, right));
            acc
        }
    )
        .map_err(|e| match e {
            Failure(Code(_, Custom(UnclosedOperation))) => Failure(Code(input, Custom(UnclosedOperation))),
            e => e,
        })
}

fn operation(input: Slice) -> nom::IResult<Slice, (InfixOperator, Operand), ErrorKind> {
    let (base, _) = multispace0(input).expect("multispace0");
    let (next, op) = infix_operator(base)?;
    if (op == InfixOperator::Sub || op == InfixOperator::Mod) && is_end_of_expression(base) {
        Err(Error(Code(input, Custom(NotRecognized))))?;
    }
    let (next, _) = multispace0(next).expect("multispace0");
    let (output, right) = operand(next).map_err(|e| match e {
        Error(Code(_, Custom(NotRecognized))) => Failure(Code(input, Custom(UnclosedOperation))),
        e => e,
    })?;
    Ok((output, (op, right)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good_fold(sample: &str, check: Operations) {
        assert_eq!(
            Ok((Slice(EMPTY), check)),
            operation_fold0(Slice(sample.as_bytes())),
        );
    }

    fn good_fold_rest(sample: &str, rest: &str, check: Operations) {
        assert_eq!(
            Ok((Slice(rest.as_bytes()), check)),
            operation_fold0(Slice(sample.as_bytes())),
        );
    }

    fn good_op(sample: &str, op: InfixOperator, right: impl Into<Operand<'static>>) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), (op, right.into()))),
            operation(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            operation_fold0(sample),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedOperation)))),
            operation_fold0(sample),
        );
    }

    fn prefix(operator: PrefixOperator, operation: impl Into<Operand<'static>>) -> Operand<'static> {
        Operand::Prefix(operator, box operation.into())
    }

    fn op_simple(left: impl Into<Operand<'static>>) -> Operations<'static> {
        Operations(left.into(), vec![])
    }

    fn op_pair(
        left: impl Into<Operand<'static>>,
        operator: InfixOperator,
        right: impl Into<Operand<'static>>,
    ) -> Operations<'static> {
        Operations(left.into(), vec![(operator, right.into())])
    }

    #[test]
    fn simple() {
        good_fold(r#"42"#, op_simple(NumberLiteral(b"42")));
        good_fold(r#"-42"#, op_simple(prefix(PrefixOperator::Minus, NumberLiteral(b"42"))));
        good_fold(r#"- 42"#, op_simple(prefix(PrefixOperator::Minus, NumberLiteral(b"42"))));
        good_fold(r#"identifier"#, op_simple(Identifier(b"identifier")));
        good_fold(r#"-identifier"#, op_simple(prefix(PrefixOperator::Minus, Identifier(b"identifier"))));
        good_fold(r#"- identifier"#, op_simple(prefix(PrefixOperator::Minus, Identifier(b"identifier"))));
        good_fold_rest(r#"42-}}"#, "-}}", op_simple(NumberLiteral(b"42")));
        good_fold_rest(r#"42 -}}"#, " -}}", op_simple(NumberLiteral(b"42")));
        good_fold_rest(r#"identifier-}}"#, "-}}", op_simple(Identifier(b"identifier")));
        good_fold_rest(r#"identifier -}}"#, " -}}", op_simple(Identifier(b"identifier")));
        good_fold_rest(r#"42-%}"#, "-%}", op_simple(NumberLiteral(b"42")));
        good_fold_rest(r#"42 -%}"#, " -%}", op_simple(NumberLiteral(b"42")));
        good_fold_rest(r#"identifier-%}"#, "-%}", op_simple(Identifier(b"identifier")));
        good_fold_rest(r#"identifier -%}"#, " -%}", op_simple(Identifier(b"identifier")));

        not_recognized(r#""#);
        not_recognized(r#"_42"#);
        not_recognized(r#"-_42"#);
        not_recognized(r#"- _42"#);
        not_recognized(r#"_identifier"#);
        not_recognized(r#"-_identifier"#);
        not_recognized(r#"- _identifier"#);
    }

    #[test]
    fn next_part() {
        good_op(r#"+2"#, InfixOperator::Add, NumberLiteral(b"2"));
        good_op(r#"+ 2"#, InfixOperator::Add, NumberLiteral(b"2"));
    }

    #[test]
    fn complex() {
        good_fold(r#"1+2"#, op_pair(NumberLiteral(b"1"),InfixOperator::Add, NumberLiteral(b"2")));
        good_fold(r#"1 + 2"#, op_pair(NumberLiteral(b"1"),InfixOperator::Add, NumberLiteral(b"2")));
        good_fold(r#"1  +  2"#, op_pair(NumberLiteral(b"1"),InfixOperator::Add, NumberLiteral(b"2")));
        good_fold(r#"6 * 7"#, op_pair(NumberLiteral(b"6"),InfixOperator::Mul, NumberLiteral(b"7")));
        good_fold(r#"x == 0"#, op_pair(Identifier(b"x"),InfixOperator::Eq, NumberLiteral(b"0")));
        good_fold(r#"0 == x"#, op_pair(NumberLiteral(b"0"),InfixOperator::Eq, Identifier(b"x")));

        not_recognized(r#"_1 + 2"#);

        unclosed(r#"1 +"#);
        unclosed(r#"1 + _2"#);
        unclosed(r#"1+}}"#);
        unclosed(r#"1+ }}"#);
        unclosed(r#"1 +}}"#);
        unclosed(r#"1 + }}"#);
        unclosed(r#"1- }}"#);
        unclosed(r#"1 - }}"#);
        unclosed(r#"1% }}"#);
        unclosed(r#"1 % }}"#);
    }
}
