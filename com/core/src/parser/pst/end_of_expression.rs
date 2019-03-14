use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
    multispace,
};

use crate::container::pst::{self, ErrorKind::*};

pub fn end_of_expression(input: Slice) -> nom::IResult<Slice, (), pst::ErrorKind> {
    if is_end_of_expression(input) {
        Ok((input, ()))
    } else {
        Err(Error(Code(input, Custom(NotRecognized))))
    }
}

pub fn is_end_of_expression(input: Slice) -> bool {
    let res: nom::IResult<Slice, ()> = peek!(input,
        do_parse!(
            opt!(multispace) >>
            alt!(
                tag!("|") |
                tag!("%}") |
                tag!("}}") |
                eof!()
            ) >>
            (())
        )
    );
    res.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn end(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            true,
            is_end_of_expression(sample),
        );
    }

    fn not_end(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            false,
            is_end_of_expression(sample),
        );
    }

    #[test]
    fn test() {
        end(r#""#);
        end(r#" "#);
        end(r#"|"#);
        end(r#" |"#);
        end(r#"%}"#);
        end(r#" %}"#);
        end(r#"}}"#);
        end(r#" }}"#);

        not_end(r#"123"#);
        not_end(r#" 123"#);
        not_end(r#"identifier"#);
        not_end(r#" identifier"#);
        not_end(r#""string""#);
        not_end(r#" "string""#);
        not_end(r#"+"#);
        not_end(r#" +"#);
    }
}
