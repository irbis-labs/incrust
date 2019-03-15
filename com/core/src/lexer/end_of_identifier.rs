use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
};

use crate::container::pst::{self, ErrorKind::*};
use crate::lexer::end_of_expression::is_end_of_expression;

pub fn end_of_identifier(input: Slice) -> nom::IResult<Slice, (), pst::ErrorKind> {
    if is_end_of_identifier(input) {
        Ok((input, ()))
    } else {
        Err(Error(Code(input, Custom(NotRecognized))))
    }
}

pub fn is_end_of_identifier(input: Slice) -> bool {
    let res: nom::IResult<Slice, _> = peek!(input,
        alt!(
            char!(' ') => { drop } |
            char!('\n') => { drop } |
            char!('\t') => { drop } |
            char!('\r') => { drop } |
            char!('\r') => { drop } |
            char!('+') => { drop } |
            char!('-') => { drop } |
            char!('*') => { drop } |
            char!('/') => { drop } |
            char!('%') => { drop } |
            char!('=') => { drop } |
            char!('<') => { drop } |
            char!('>') => { drop } |
            char!('(') => { drop } |
            char!(')') => { drop } |
            char!('[') => { drop } |
            char!(']') => { drop } |
            char!('{') => { drop } |
            char!('}') => { drop } |
            char!(':') => { drop } |
            char!('.') => { drop } |
            char!(',') => { drop } |
            char!(';') => { drop } |
            char!('"') => { drop } |
            char!('\'') => { drop }
        )
    );
    res.is_ok() || is_end_of_expression(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn end(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            true,
            is_end_of_identifier(sample),
        );
    }

    fn not_end(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            false,
            is_end_of_identifier(sample),
        );
    }

    #[test]
    fn test() {
        end(r#""#);
        end(" ");
        end("\n");
        end("\t");
        end("\r");
        end("\r");
        end("+");
        end("-");
        end("*");
        end("/");
        end("%");
        end("=");
        end("<");
        end(">");
        end("(");
        end(")");
        end("[");
        end("]");
        end("{");
        end("}");
        end(":");
        end(".");
        end(",");
        end(";");
        end("'");
        end("\"");
        end("|");

        not_end(r#"0"#);
        not_end(r#"_"#);
        not_end(r#"identifier"#);
    }
}
