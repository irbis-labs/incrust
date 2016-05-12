use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Statement};




named!(pub statement<&[u8], Parsed>,
    alt!(
        raw_block
    )
);

// --------------------------------------------------------------------------------------------------------------------

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

#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    use nom::IResult::Done;

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
