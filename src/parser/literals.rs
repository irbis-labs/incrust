use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Expression, Literal};




pub fn literal(input: &[u8]) -> IResult<&[u8], Expression> {
    let (i, l) = try_parse!(input, alt!(str_lit) );
    IResult::Done(i, Expression::Literal(l))
}

// --------------------------------------------------------------------------------------------------------------------

//fn char_literal(input: &[u8]) -> IResult<&[u8], Literal> {
//    let (i, (_, s, _)) = try_parse!(input,
//        tuple!(
//            char!('\''),
//            is_not!(r"'"),
//            char!('\''),
//        )
//    );
//
//    IResult::Done(i, Literal::Char(s))
//}

pub fn str_lit(input: &[u8]) -> IResult<&[u8], Literal> {
    let (i, (_, s, _)) = try_parse!(input,
        tuple!(
            char!('"'),
            str_char_agg,
            char!('"')
        )
    );
    IResult::Done(i, Literal::Str(s))
}


named!(pub str_char_agg<&[u8], String>,
    chain!(
        c: many0!(alt!( str_char_escaped | str_char )),
        || c.join("")
    )
);

named!(pub str_char<&[u8], &str>, map_res!( is_not!("\\\""), str::from_utf8 ) );

named!(pub str_char_escaped<&[u8], &str>, chain!(
    char!('\\')         ~
    c: alt!(
        char!('\\') |
        char!('\'') |
        char!('\"') |
        char!('t')  |
        char!('r')  |
        char!('n')
    )                   ,
    || match c {
        '\\' => "\\",
        // '\'' => "\'",
        '\"' => "\"",
        't'  => "\t",
        'r'  => "\r",
        'n'  => "\n",
        _  => unreachable!()
    }
));


// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    use nom::IResult::Done;

    #[test]
    fn literal_str_char() {
        assert_eq!(Done(&b""[..], r#"\"#), super::str_char_escaped(br#"\\"#));
        assert_eq!(Done(&b""[..], r#"""#), super::str_char_escaped(br#"\""#));
        assert_eq!(Done(&b""[..], "\t"),   super::str_char_escaped(br#"\t"#));
        assert_eq!(Done(&b""[..], "\r"),   super::str_char_escaped(br#"\r"#));
        assert_eq!(Done(&b""[..], "\n"),   super::str_char_escaped(br#"\n"#));
    }

    #[test]
    fn literal_str() {
        use ::template::Literal::Str;
        assert_eq!(Done(&b""[..], Str(r#" {{ "#.into()).into()),        super::literal(br#"" {{ ""#));
        assert_eq!(Done(&b""[..], Str(r#"{{"#.into()).into()),          super::literal(br#""{{""#));
        assert_eq!(Done(&b""[..], Str("New\nline".into()).into()),      super::literal(br#""New\nline""#));
        assert_eq!(Done(&b""[..], Str(r#""Quotes""#.into()).into()),    super::literal(br#""\"Quotes\"""#));
    }
}
