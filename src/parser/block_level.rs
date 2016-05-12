use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Mustache};

use super::*;


named!(pub text<&[u8], Vec<Parsed> >,
    chain!(
        t: many0!(parsed)   ~
        eof                 ,
        || { t }
    )
);


named!(parsed<&[u8], Parsed>,
    alt!(
        comment             |
        statement           |
        mustache            |
        start_mustache      |
        not_start_mustache
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
            opt!(multispace),
            full_expression,
            opt!(multispace),
            tag!("}}")
        )
    );
    IResult::Done(i, Mustache::new(fe).into())
}

fn not_start_mustache(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (i, text) = try_parse!(input,
        tuple!(
            map_res!(
                is_not!("{"),
                str::from_utf8
            )
        )
    );
    IResult::Done(i, Parsed::Text(text.to_owned()))
}

fn start_mustache(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (i, text) = try_parse!(input,
        map_res!(
            tag!("{"),
            str::from_utf8
        )
    );
    IResult::Done(i, Parsed::Text(text.to_owned()))
}


// ---------------------------------------------------------------------------


//#[cfg(test)]
//mod tests {
//    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
//
//    use nom::IResult::Done;
//
//}
