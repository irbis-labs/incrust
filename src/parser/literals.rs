#![cfg_attr(feature = "clippy", allow(many_single_char_names))]

use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, space, multispace};

use container::expression::Factor;
use container::expression::Literal;


pub fn literal(input: &[u8]) -> IResult<&[u8], Factor> {
    let (i, l) = try_parse!(input, alt!(lit_str | lit_char | lit_num) );
    IResult::Done(i, Factor::Literal(l))
}

// --------------------------------------------------------------------------------------------------------------------

pub fn lit_num(input: &[u8]) -> IResult<&[u8], Literal> {
    let (i, s) = try_parse!(input, map_res!( is_a!("0123456789."), str::from_utf8 ) );
    match parse_int(s).or_else(|_| parse_float(s)) {
        Ok(res) => IResult::Done(i, res),
        Err(err) => IResult::Error(NomErr::Code(ErrorKind::Custom(err))),
    }
}

fn parse_float(s: &str) -> Result<Literal, u32> {
    let n: f64 = s.parse().map_err(|_| 1301_u32)?;
    Ok(Literal::Real(n))
}

fn parse_int(s: &str) -> Result<Literal, u32> {
    let n: i64 = s.parse().map_err(|_| 1302_u32)?;
    Ok(Literal::Int(n))
}

// ---------------------------------------------------------------------------

fn lit_char(input: &[u8]) -> IResult<&[u8], Literal> {
    let (i, (_, s, _)) = try_parse!(input,
        tuple!(
            char!('\''),
            char_char_agg,
            char!('\'')
        )
    );
    trace!("lit_char {:?}", s);
    match parse_char(s.as_str()) {
        Ok(chr) => IResult::Done(i, Literal::Char(chr)),
        Err(err) => IResult::Error(NomErr::Code(ErrorKind::Custom(err))),
    }
}

named!(pub char_char_agg<&[u8], String>,
    chain!(
        c: many0!(alt!( char_escaped | char_char )),
        || c.join("")
    )
);

named!(pub char_char<&[u8], &str>, map_res!( is_not!(r#"\'"#), str::from_utf8 ) );

named!(pub char_escaped<&[u8], &str>, chain!(
    char!('\\')         ~
    c: alt!(
        char!('\\') |
        char!('\'') |
        char!('"')  |
        char!('t')  |
        char!('r')  |
        char!('n')
    )                   ,
    || match c {
        '\\' => "\\",
        '\'' => "\'",
        '"'  => "\"",
        't'  => "\t",
        'r'  => "\r",
        'n'  => "\n",
        _  => unreachable!()
    }
));

fn parse_char(s: &str) -> Result<char, u32> {
    let mut i = s.chars();
    let (a, b) = (i.next(), i.next());
    match a {
        None => Err(1303u32),
        Some(c) => match b {
            Some(_) => Err(1304u32),
            None => Ok(c)
        },
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub fn lit_str(input: &[u8]) -> IResult<&[u8], Literal> {
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
        c: many0!(alt!( char_escaped | str_char )),
        || c.join("")
    )
);

named!(pub str_char<&[u8], &str>, map_res!( is_not!(r#"\""#), str::from_utf8 ) );

// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use nom::IResult::Done;

    #[test]
    fn literal_str_char() {
        assert_eq!(Done(&b""[..], r#"\"#), super::char_escaped(br#"\\"#));
        assert_eq!(Done(&b""[..], r#"""#), super::char_escaped(br#"\""#));
        assert_eq!(Done(&b""[..], "\t"),   super::char_escaped(br#"\t"#));
        assert_eq!(Done(&b""[..], "\r"),   super::char_escaped(br#"\r"#));
        assert_eq!(Done(&b""[..], "\n"),   super::char_escaped(br#"\n"#));
    }

    #[test]
    fn literal_str() {
        use container::expression::Literal::Str;
        assert_eq!(Done(&b""[..], Str(r#" {{ "#.into()).into()),        super::lit_str(br#"" {{ ""#));
        assert_eq!(Done(&b""[..], Str(r#" {{ "#.into()).into()),        super::literal(br#"" {{ ""#));
        assert_eq!(Done(&b""[..], Str(r#"{{"#.into()).into()),          super::literal(br#""{{""#));
        assert_eq!(Done(&b""[..], Str("New\nline".into()).into()),      super::literal(br#""New\nline""#));
        assert_eq!(Done(&b""[..], Str(r#""Quotes""#.into()).into()),    super::literal(br#""\"Quotes\"""#));
    }

    #[test]
    fn literal_char() {
        use container::expression::Literal::Char;
        assert_eq!("\\\\",                                  r#"\\"#);

        assert_eq!(Done(&b""[..], r#"\"#),                  super::char_escaped(br#"\\"#));

        assert_eq!(Ok('\\'),                                super::parse_char("\\"));
        assert_eq!(Ok('\n'),                                super::parse_char("\n"));

        assert_eq!(Done(&b""[..], Char('\\').into()),       super::lit_char(br#"'\\'"#));
        assert_eq!(Done(&b""[..], Char('\'').into()),       super::lit_char(br#"'\''"#));
        assert_eq!(Done(&b""[..], Char('"') .into()),       super::lit_char(br#"'\"'"#));
        assert_eq!(Done(&b""[..], Char('"') .into()),       super::lit_char(br#"'"'"#));
        assert_eq!(Done(&b""[..], Char('\t').into()),       super::lit_char(br#"'\t'"#));
        assert_eq!(Done(&b""[..], Char('\r').into()),       super::lit_char(br#"'\r'"#));
        assert_eq!(Done(&b""[..], Char('\n').into()),       super::lit_char(br#"'\n'"#));
        assert_eq!(Done(&b""[..], Char(' ') .into()),       super::lit_char(br#"' '"#));

        assert!(super::lit_char(br#"''"#).is_err());
        assert!(super::lit_char(br#"'  '"#).is_err());
    }

    #[test]
    fn literal_int() {
        use container::expression::Literal::Int;
        assert_eq!(Ok(Int(42)),                             super::parse_int("42"));
        assert_eq!(Done(&b""[..], Int(42).into()),          super::lit_num(b"42"));
        assert_eq!(Done(&b""[..], Int(42).into()),          super::literal(b"42"));
    }

    #[test]
    fn literal_real() {
        #![cfg_attr(feature = "clippy", allow(approx_constant))]
        use container::expression::Literal::Real;
        assert_eq!(Ok(Real(3.1415926)),                     super::parse_float("3.1415926"));
        assert_eq!(Ok(Real(0.1)),                           super::parse_float(".1"));
        assert_eq!(Ok(Real(1.0)),                           super::parse_float("1."));

        assert_eq!(Done(&b""[..], Real(3.1415926).into()),  super::lit_num(b"3.1415926"));
        assert_eq!(Done(&b""[..], Real(0.1).into()),        super::lit_num(b".1"));
        assert_eq!(Done(&b""[..], Real(1.0).into()),        super::lit_num(b"1."));

        assert_eq!(Done(&b""[..], Real(3.1415926).into()),  super::literal(b"3.1415926"));
        assert_eq!(Done(&b""[..], Real(0.1).into()),        super::literal(b".1"));
        assert_eq!(Done(&b""[..], Real(1.0).into()),        super::literal(b"1."));
    }
}
