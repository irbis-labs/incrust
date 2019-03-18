use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{*, ErrorKind::*};
use crate::lexer::*;

pub fn expression_tag(input: Slice) -> nom::IResult<Slice, ExpressionTag, ErrorKind> {
    let (next, _tag1) = complete!(input, tag!("{{"))
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    let (next, strip_before) = opt!(next, tag!("-"))
        .map(|(o, sb)| (o, sb.is_some()))
        .expect("strip left");

    let (next, _) = nom::multispace0(next).expect("spaces");

    let (next, expression) = operation_fold0(next)
        .map_err(|_| Failure(Code(input, Custom(IncorrectExpression))))?;

    let (next, _) = nom::multispace0(next).expect("spaces");

    let (next, strip_after) = opt!(next, tag!("-"))
        .map(|(o, sa)| (o, sa.is_some()))
        .expect("strip right");

    let (output, _tag2) = tag!(next, "}}")
        .map_err(|_| Failure(Code(input, Custom(UnclosedExpressionTag))))?;

    Ok((output, ExpressionTag::new(expression, strip_before, strip_after)))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, expression: Operations, strip_before: bool, strip_after: bool) {
        assert_eq!(
            Ok((Slice(EMPTY), ExpressionTag::new(expression, strip_before, strip_after))),
            expression_tag(Slice(sample.as_bytes())),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedExpressionTag)))),
            expression_tag(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            expression_tag(sample),
        );
    }

    fn incorrect(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(IncorrectExpression)))),
            expression_tag(sample),
        );
    }

    #[test]
    fn simple() {
        let expr = Operations(Identifier(b"identifier").into(), vec![]);

        good("{{identifier}}", expr.clone(), false, false);
        good("{{-identifier}}", expr.clone(), true, false);
        good("{{identifier-}}", expr.clone(), false, true);
        good("{{-identifier-}}", expr.clone(), true, true);

        good("{{ identifier }}", expr.clone(), false, false);
        good("{{- identifier }}", expr.clone(), true, false);
        good("{{ identifier -}}", expr.clone(), false, true);
        good("{{- identifier -}}", expr.clone(), true, true);

        not_recognized("");
        not_recognized("{");
        not_recognized("plain text");
        not_recognized("{# comment #}");
        not_recognized("{% statement %}");

        incorrect("{{");
        incorrect("{{ ");
        incorrect("{{}}");
        incorrect("{{ }}");
        incorrect("{{ _ }}");

        // TODO maybe it should be an unclosed?
        incorrect("{{ identifier-");
        incorrect("{{ identifier%");
        incorrect("{{ identifier -");
        incorrect("{{ identifier %");
        incorrect("{{ identifier-}");
        incorrect("{{ identifier -}");

        unclosed("{{ unclosed");
        unclosed("{{ unclosed}");
        unclosed("{{ unclosed }");
        unclosed("{{ unclosed%}");
        unclosed("{{ unclosed %}");
        unclosed("{{ unclosed-%}");
        unclosed("{{ unclosed -%}");
    }

    #[test]
    fn complex() {
        let expr = Operations(
            NumberLiteral(b"1").into(),
            vec![(InfixOperator::Add, NumberLiteral(b"2").into())]
        );

        good("{{1 + 2}}", expr.clone(), false, false);
        good("{{-1 + 2}}", expr.clone(), true, false);
        good("{{1 + 2-}}", expr.clone(), false, true);
        good("{{-1 + 2-}}", expr.clone(), true, true);

        good("{{ 1 + 2 }}", expr.clone(), false, false);
        good("{{- 1 + 2 }}", expr.clone(), true, false);
        good("{{ 1 + 2 -}}", expr.clone(), false, true);
        good("{{- 1 + 2 -}}", expr.clone(), true, true);

        incorrect("{{ 1 + 2-");
        incorrect("{{ 1 + 2%");
        incorrect("{{ 1 + 2 -");
        incorrect("{{ 1 + 2 %");

        unclosed("{{ 1 + 2");
        unclosed("{{ 1 + 2}");
        unclosed("{{ 1 + 2%}");
        unclosed("{{ 1 + 2-%}");
        unclosed("{{ 1 + 2 ");
        unclosed("{{ 1 + 2 }");
        unclosed("{{ 1 + 2 %}");
        unclosed("{{ 1 + 2 -%}");
    }
}
