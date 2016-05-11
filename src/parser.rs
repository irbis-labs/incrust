//use std::collections::hash_map::{Entry};
use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Mustache, Expression, FullExpression, FilterItem, Literal, Statement};

// ---------------------------------------------------------------------------

macro_rules! stmt {
    ( $i:expr, $cmd: expr ) => ({
        use nom::multispace;
        chaining_parser!($i, 0usize,
            tag!("{%")              ~
            opt!(multispace)        ~
            tag!($cmd)              ~
            opt!(multispace)        ~
            tag!("%}")              ,
            || -> Statement { ().into() }
        )
    });
}


/// `take_till!(T -> bool) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes until the provided function succeeds
///
/// The argument is either a function `&[T] -> bool` or a macro returning a `bool
#[macro_export]
macro_rules! take_till_slc (
  ($input:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use nom::InputLength;
      match $input.iter().enumerate().position(|(i, _)| $submac!(&$input[i..], $($args)*)) {
        Some(n) => IResult::Done(&$input[n..], &$input[..n]),
        None    => IResult::Done(&$input[($input).input_len()..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    take_till_slc!($input, call!($f));
  );
);


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
        raw_block           |
        mustache            |
        start_mustache      |
        not_start_mustache
    )
);


// ---------------------------------------------------------------------------

named!(pub stmt_raw<&[u8], Statement>, stmt!("raw"));
named!(pub stmt_endraw<&[u8], Statement>, stmt!("endraw"));

#[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
fn raw_block(input: &[u8]) -> IResult<&[u8], Parsed> {
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    fn is_end<'a>(input: &'a [u8]) -> bool {
        let b = || -> IResult<&'a [u8], ()> {
            let _ = try_parse!(input, stmt_endraw );
            IResult::Done(input, ())
        };
        b().is_done()
    }

    let (i, txt) = try_parse!(input,
        chain!(
            stmt_raw                    ~
            raw: map_res!(
                take_till_slc!(is_end),
                str::from_utf8
            )                           ~
            stmt_endraw                 ,
            || Parsed::Text(raw.to_owned())
        )
    );
    IResult::Done(i, txt)
}

#[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
fn for_block(input: &[u8]) -> IResult<&[u8], Parsed> {
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    fn is_end<'a>(input: &'a [u8]) -> bool {
        let b = || -> IResult<&'a [u8], ()> {
            let _ = try_parse!(input, stmt!("endfor") );
            IResult::Done(input, ())
        };
        b().is_done()
    }

    let (i, txt) = try_parse!(input,
        chain!(
            stmt!("for")                ~
            raw: map_res!(
                take_till_slc!(is_end),
                str::from_utf8
            )                           ~
            stmt!("endfor")             ,
            || Parsed::Text(raw.to_owned())
        )
    );
    IResult::Done(i, txt)
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

// ---------------------------------------------------------------------------

fn full_expression(input: &[u8]) -> IResult<&[u8], FullExpression> {
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

fn literal(input: &[u8]) -> IResult<&[u8], Expression> {
    let (i, l) = try_parse!(input, alt!(str_lit) );
    IResult::Done(i, Expression::Literal(l))
}

// ---------------------------------------------------------------------------

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

named!(pub str_char<&[u8], &str>, map_res!( is_not!("\\\""), str::from_utf8 ) );

named!(pub str_char_agg<&[u8], String>,
    chain!(
        c: many0!(alt!( str_char_escaped | str_char )),
        || c.join("")
    )
);

fn str_lit(input: &[u8]) -> IResult<&[u8], Literal> {
    let (i, (_, s, _)) = try_parse!(input,
        tuple!(
            char!('"'),
            str_char_agg,
            char!('"')
        )
    );
    IResult::Done(i, Literal::Str(s))
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

    #[test]
    fn format() {
        use ::template::FilterItem::Simple;
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"| e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|  e"));
        assert!(super::filter(b"| 1wrong").is_err());
        assert!(super::filter(b"| _wrong").is_err());
    }

    #[test]
    fn statement() {
        use ::template::Statement;
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{%raw%}"));
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{% raw %}"));
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{%  raw  %}"));
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{%\traw\t%}"));
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{%\nraw\n%}"));
        assert_eq!(Done(&b""[..], Statement{}),  super::stmt_raw(b"{%\rraw\r%}"));
    }

    #[test]
    fn raw() {
        use ::template::Parsed::Text;
        assert_eq!(Done(&b""[..], Text("{{ raw }}".into())), super::raw_block(b"{% raw %}{{ raw }}{% endraw %}"));
        assert_eq!(Done(&b""[..], Text("{% if %}".into())),  super::raw_block(b"{% raw %}{% if %}{% endraw %}"));
        assert_eq!(Done(&b""[..], Text("{% if %}".into())),  super::raw_block(b"{%  raw %}{% if %}{%  endraw %}"));
    }

    #[test]
    fn foreach() {
        use ::template::Parsed::Text;
        assert_eq!(Done(&b""[..], Text("{{ i }}".into())), super::for_block(b"{% for %}{{ i }}{% endfor %}"));
    }
}
