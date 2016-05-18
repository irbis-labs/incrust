use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Mustache};

use super::statements::{statement, stmt_edge};
use super::expressions::{full_expression};


named!(pub text<&[u8], Vec<Parsed> >, chain!( t: inner ~ eof, || t ) );


pub fn inner(input: &[u8]) -> IResult<&[u8], Vec<Parsed> > {
    let (i, list) = try_parse!(input, many0!(parsed) );
    IResult::Done(i, list)
}



named!(parsed<&[u8], Parsed>,
    alt!(
        plain_text          |
        comment             |
        mustache            |
        statement
    )
);


fn comment(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (i, (_, comment, _)) = try_parse!(input,
        tuple!(
            tag!("{#"),
            map_res!(
                take_until!("#}"),
                str::from_utf8
            ),
            tag!("#}")
        )
    );
    IResult::Done(i, Parsed::Comment(comment.to_owned()))
}


fn mustache(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (i, (_, _, fe, _, _)) = try_parse!(input,
        tuple!(
            tag!("{{"),
            many0!(multispace),
            full_expression,
            many0!(multispace),
            tag!("}}")
        )
    );
    IResult::Done(i, Mustache::new(fe).into())
}


fn plain_text(input: &[u8]) -> IResult<&[u8], Parsed> {
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    fn try_brace<'a>(input: &'a [u8]) -> IResult<&[u8], &str> {
        let b = || -> IResult<&'a [u8], ()> {
            let _ = try_parse!(input, alt!( mustache | comment | stmt_edge ) );
            IResult::Done(input, ())
        };
        match b().is_done() {
            false   => IResult::Done(&input[1..], "{"),
            true    => IResult::Error(NomErr::Code(ErrorKind::Custom(0))),
        }
    }

    let (i, text) = try_parse!(input,
        chain!( v: many1!(
            alt!( map_res!( is_not!("{"), str::from_utf8 ) | try_brace )
        ), || v.join("") )
    );
    IResult::Done(i, Parsed::Text(text))
}



// ---------------------------------------------------------------------------


//#[cfg(test)]
//mod tests {
//    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
//
//    use nom::IResult::Done;
//
//}
