use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{self, ErrorKind::*};
use crate::parser::pst::*;

fn b2u(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

pub fn statement(input: Slice) -> nom::IResult<Slice, pst::Node, pst::ErrorKind> {
    let (next, tag1) = complete!(input, tag!("{%"))
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    let (next, strip_before) = opt!(next, tag!("-"))
        .map(|(o, sb)| (o, sb.is_some()))
        .expect("spaces");

    let (next, stmt) = statement_expression(next)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStatement))))?;

    let (next, strip_after) = opt!(next, tag!("-"))
        .map(|(o, sa)| (o, sa.is_some()))
        .expect("spaces");

    let (output, tag2) = tag!(next, "%}")
        .map_err(|_| Failure(Code(input, Custom(UnclosedStatement))))?;

    Ok((output, pst::Node::Statement(box stmt, strip_before, strip_after)))
}

fn statement_expression(input: Slice) -> nom::IResult<Slice, pst::Node, pst::ErrorKind> {
    let (next, _) = opt!(input, nom::multispace).expect("spaces");

    let (next, ident) = identifier(next)
        .map_err(|_| Failure(Code(input, Custom(IncorrectStatement))))?;

    let (output, _) = opt!(next, nom::multispace).expect("spaces");

    Ok((output, pst::Node::StatementExpression(box ident)))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, ident: &[u8], strip_before: bool, strip_after: bool) {
        let sample = Slice(sample.as_bytes());
        let ident = box pst::Node::Identifier(ident);
        let stmt = box pst::Node::StatementExpression(ident);
        assert_eq!(
            Ok((Slice(EMPTY), pst::Node::Statement(stmt, strip_before, strip_after))),
            statement(sample),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedStatement)))),
            statement(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            statement(sample),
        );
    }

    fn incorrect(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(IncorrectStatement)))),
            statement(sample),
        );
    }

    #[test]
    fn test() {
        good("{%raw%}", b"raw", false, false);
        good("{%-raw%}", b"raw", true, false);
        good("{%raw-%}", b"raw", false, true);
        good("{%-raw-%}", b"raw", true, true);

        good("{% raw %}", b"raw", false, false);
        good("{%- raw %}", b"raw", true, false);
        good("{% raw -%}", b"raw", false, true);
        good("{%- raw -%}", b"raw", true, true);

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
    }
}
