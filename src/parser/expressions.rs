use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Expression, FullExpression, FilterItem};

use super::literals::literal;


// ---------------------------------------------------------------------------

pub fn full_expression(input: &[u8]) -> IResult<&[u8], FullExpression> {
    let (i, (expr, filters)) = try_parse!(input,
        tuple!(
            expression,
            filter_agg
        )
    );
    IResult::Done(i, FullExpression::new(expr, filters))
}

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

fn expression(input: &[u8]) -> IResult<&[u8], Expression> {
    let (i, (_, expr)) = try_parse!(input, tuple!( opt!(un_op), expression_item ) );
    IResult::Done(i, expr)
}

fn expression_item(input: &[u8]) -> IResult<&[u8], Expression> {
    let (i, expr) = try_parse!(input, alt!( literal | variable ) );
    IResult::Done(i, expr)
}

fn variable(input: &[u8]) -> IResult<&[u8], Expression> {
    let (i, id) = try_parse!(input, identifier );
    IResult::Done(i, Expression::Variable(id))
}

// ---------------------------------------------------------------------------

named!(pub un_plus<&[u8], char>, char!('+') );
named!(pub un_minus<&[u8], char>, char!('-') );

fn un_op(input: &[u8]) -> IResult<&[u8], char> {
    let (i, (op, _)) = try_parse!(input,
        tuple!(
            alt!( un_plus | un_minus ),
            opt!(multispace)
        )
    );
    IResult::Done(i, op)
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
        assert_eq!(Done(&b""[..], "var_1".to_owned()),    super::identifier(b"var_1"));
        assert!(super::identifier(b"_wrong").is_err());
        assert!(super::identifier(b"1wrong").is_err());
    }

    #[test]
    fn format() {
        use ::template::FilterItem::Simple;
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"| e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|  e"));
        assert!(super::filter(b"| 1wrong").is_err());
        assert!(super::filter(b"| _wrong").is_err());
    }
}
