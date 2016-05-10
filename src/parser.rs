//use std::collections::hash_map::{Entry};
use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Mustache, Expression, FilterItem};


// ---------------------------------------------------------------------------

named!(pub text<&[u8], Vec<Parsed> >,
    chain!(
        t: many0!(parsed)   ~
        eof                 ,
        || { t }
    )
);

named!(parsed<&[u8], Parsed>,
    alt_complete!(
        comment             |
        mustache            |
        start_mustache      |
        not_start_mustache
    )
);


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
    let (i, (_, _, id, filters, _, _)) = try_parse!(input,
        tuple!(
            tag!("{{"),
            opt!(multispace),
            identifier,
            filter_agg,
            opt!(multispace),
            tag!("}}")
        )
    );
    IResult::Done(i, Mustache::new(Expression::Variable(id), filters).into())
}

// ---------------------------------------------------------------------------

named!(pub filter_agg<&[u8], Vec<FilterItem> >,
    many0!(chain!(
        many0!(multispace) ~
        f: filter,
        || f
    ))
);


fn filter(input: &[u8]) -> IResult<&[u8], FilterItem> {
    let (i, (_, _, id)) = try_parse!(input,
        tuple!(
            tag!("|"),
            opt!(multispace),
            identifier
        )
    );
    IResult::Done(i, FilterItem::Simple(id))
}

// ---------------------------------------------------------------------------

#[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
fn identifier(input: &[u8]) -> IResult<&[u8], String> {
    let (i, id) = try_parse!(input,
        chain!(
            start: map_res!(alpha, str::from_utf8)~
            rest: many0!(map_res!(alt!(tag!("_") | alphanumeric), str::from_utf8)),
            || {
                rest.into_iter().fold(start.to_string(), |mut acc, slice| {
                    acc.push_str(slice);
                    acc
                })
            }
        )
    );
    IResult::Done(i, id)
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    use nom::IResult::Done;

    #[test]
    fn identifier() {
        assert_eq!(Done(&b""[..], "var_name".to_owned()), super::identifier(b"var_name"));
        assert_eq!(Done(&b""[..], "var_1".to_owned()), super::identifier(b"var_1"));
        assert!(super::identifier(b"_wrong").is_err());
        assert!(super::identifier(b"1wrong").is_err());
    }

    #[test]
    fn filter() {
        use ::template::FilterItem::Simple;
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"| e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|  e"));
        assert!(super::filter(b"| 1wrong").is_err());
        assert!(super::filter(b"| _wrong").is_err());
    }
}
