use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{*, ErrorKind::*};
use crate::lexer::*;

pub fn statement_tag(input: Slice) -> nom::IResult<Slice, StatementTag, ErrorKind> {
    let (next, _tag1) = complete!(input, tag!("{%"))
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    let (next, strip_before) = opt!(next, tag!("-"))
        .map(|(o, sb)| (o, sb.is_some()))
        .expect("strip left");

    let (next, _) = nom::multispace0(next).expect("spaces");

    let (next, stmt) = statement_expression(next)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStatement))))?;

    let (next, _) = nom::multispace0(next).expect("spaces");

    let (next, strip_after) = opt!(next, tag!("-"))
        .map(|(o, sa)| (o, sa.is_some()))
        .expect("strip right");

    let (output, _tag2) = tag!(next, "%}")
        .map_err(|_| Failure(Code(input, Custom(UnclosedStatementTag))))?;

    Ok((output, StatementTag::new(stmt, strip_before, strip_after)))
}

fn statement_expression(input: Slice) -> nom::IResult<Slice, StatementExpression, ErrorKind> {
    let (next, ident) = identifier(input)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStatement))))?;

    let (next, _) = nom::multispace0(next).expect("spaces");

    let (output, ops) = opt!(next, operation_fold0)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStatement))))?;

    Ok((output, StatementExpression(ident, ops)))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good_simple(sample: &str, ident: &[u8], strip_before: bool, strip_after: bool) {
        let expr = StatementExpression(Identifier(ident), None);
        good(sample, expr, strip_before, strip_after)
    }

    fn good_expr(sample: &str, ident: &[u8], ops: Operations, strip_before: bool, strip_after: bool) {
        let expr = StatementExpression(Identifier(ident), Some(ops));
        good(sample, expr, strip_before, strip_after)
    }

    fn good(sample: &str, expr: StatementExpression, strip_before: bool, strip_after: bool) {
        assert_eq!(
            Ok((Slice(EMPTY), StatementTag::new(expr, strip_before, strip_after))),
            statement_tag(Slice(sample.as_bytes())),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedStatementTag)))),
            statement_tag(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            statement_tag(sample),
        );
    }

    fn incorrect(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(IncorrectStatement)))),
            statement_tag(sample),
        );
    }

    #[test]
    fn without_expr() {
        good_simple("{%raw%}", b"raw", false, false);
        good_simple("{%-raw%}", b"raw", true, false);
        good_simple("{%raw-%}", b"raw", false, true);
        good_simple("{%-raw-%}", b"raw", true, true);

        good_simple("{% raw %}", b"raw", false, false);
        good_simple("{%- raw %}", b"raw", true, false);
        good_simple("{% raw -%}", b"raw", false, true);
        good_simple("{%- raw -%}", b"raw", true, true);

        not_recognized("");
        not_recognized("{");
        not_recognized("plain text");
        not_recognized("{# comment #}");
        not_recognized("{{ expression }}");

        incorrect("{%");
        incorrect("{% ");
        incorrect("{%%}");
        incorrect("{% %}");
        incorrect("{% _ %}");
        incorrect("{% 0 %}");

        unclosed("{% unclosed");
        unclosed("{% unclosed %");
        unclosed("{% unclosed }");
        unclosed("{% unclosed }}");

        unclosed("{%-unclosed");
        unclosed("{%-unclosed %");
        unclosed("{%-unclosed }");
        unclosed("{%-unclosed }}");

        unclosed("{%-unclosed-");
        unclosed("{%-unclosed-%");
        unclosed("{%-unclosed-}");
        unclosed("{%-unclosed-}}");

        unclosed("{%- unclosed");
        unclosed("{%- unclosed %");
        unclosed("{%- unclosed }");
        unclosed("{%- unclosed }}");

        unclosed("{%- unclosed -");
        unclosed("{%- unclosed -%");
        unclosed("{%- unclosed -}");
        unclosed("{%- unclosed -}}");
    }

    #[test]
    fn expr_simple() {
        let expr = Operations(Identifier(b"identifier").into(), vec![]);

        good_expr("{%raw identifier%}", b"raw", expr.clone(), false, false);
        good_expr("{%-raw identifier%}", b"raw", expr.clone(), true, false);
        good_expr("{%raw identifier-%}", b"raw", expr.clone(), false, true);
        good_expr("{%-raw identifier-%}", b"raw", expr.clone(), true, true);

        good_expr("{% raw identifier %}", b"raw", expr.clone(), false, false);
        good_expr("{%- raw identifier %}", b"raw", expr.clone(), true, false);
        good_expr("{% raw identifier -%}", b"raw", expr.clone(), false, true);
        good_expr("{%- raw identifier -%}", b"raw", expr.clone(), true, true);

        // TODO maybe it should be an unclosed?
        incorrect("{% unclosed identifier-");
        incorrect("{% unclosed identifier%");
        incorrect("{% unclosed identifier -");
        incorrect("{% unclosed identifier %");

        unclosed("{% unclosed identifier");
        unclosed("{% unclosed identifier}");
        unclosed("{% unclosed identifier ");
        unclosed("{% unclosed identifier }");
        unclosed("{% unclosed identifier }}");
        unclosed("{% unclosed identifier -}}");
    }

    #[test]
    fn expr_complex() {
        let expr = Operations(
            NumberLiteral(b"1").into(),
            vec![(InfixOperator::Add, NumberLiteral(b"2").into())]
        );

        good_expr("{%raw 1 + 2%}", b"raw", expr.clone(), false, false);
        good_expr("{%-raw 1 + 2%}", b"raw", expr.clone(), true, false);
        good_expr("{%raw 1 + 2-%}", b"raw", expr.clone(), false, true);
        good_expr("{%-raw 1 + 2-%}", b"raw", expr.clone(), true, true);

        good_expr("{% raw 1 + 2 %}", b"raw", expr.clone(), false, false);
        good_expr("{%- raw 1 + 2 %}", b"raw", expr.clone(), true, false);
        good_expr("{% raw 1 + 2 -%}", b"raw", expr.clone(), false, true);
        good_expr("{%- raw 1 + 2 -%}", b"raw", expr.clone(), true, true);

        incorrect("{% unclosed 1 + 2-");
        incorrect("{% unclosed 1 + 2%");
        incorrect("{% unclosed 1 + 2 -");
        incorrect("{% unclosed 1 + 2 %");

        unclosed("{% unclosed 1 + 2");
        unclosed("{% unclosed 1 + 2}");
        unclosed("{% unclosed 1 + 2}}");
        unclosed("{% unclosed 1 + 2-}}");
        unclosed("{% unclosed 1 + 2 ");
        unclosed("{% unclosed 1 + 2 }");
        unclosed("{% unclosed 1 + 2 }}");
        unclosed("{% unclosed 1 + 2 -}}");
    }
}
