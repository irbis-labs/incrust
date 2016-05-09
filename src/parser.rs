//use std::collections::hash_map::{Entry};
use std::str;
#[allow(unused_imports)]
use nom::{IResult, space, multispace};
pub use ::abc;


#[derive(Debug)]
pub struct Template {
    pub parsed: Vec<Parsed>,
}

impl Template {
    pub fn parse(templ: &str) -> abc::ParseResult {
        let parsed = parsed_aggregator(templ.as_bytes());
        println!(" == parsed == {:?}", &parsed);
        match parsed {
            IResult::Done(_, parsed) => Ok(Template { parsed: parsed }),
            IResult::Error(err) => Err(abc::ParseError::Syntax(format!("{:?}", err))),
            IResult::Incomplete(needed) => Err(abc::ParseError::Syntax(format!("Incomplete; {:?}", needed))),
        }
    }

    pub fn render(&self, args: abc::Args) -> abc::RenderResult {
        Ok(text(self.parsed.as_slice(), &args))
    }
}


fn text<'a>(tpl: &'a[Parsed], args: &'a abc::Args) -> String {
    tpl.iter().map(|x: &'a Parsed| -> &'a str {
        match *x {
            Parsed::Text(ref txt) => &txt,
            Parsed::Comment(_) => "",
            Parsed::Mustache(ref name) => match args.get(name.as_str()) {
                Some(s) => *s,
                None    => "",
            },
        }
    }).collect::<Vec<&str>>().join("")
}


#[derive(Debug)]
pub enum Parsed {
    Text(String),
    Comment(String),
    Mustache(String),
}


// ---------------------------------------------------------------------------

named!(parsed_aggregator<&[u8], Vec<Parsed> >, many0!(alt_parsed));

named!(alt_parsed<&[u8], Parsed>,
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

//fn plain_text(input: &[u8]) -> IResult<&[u8], Parsed> {
//    let (i, text) = try_parse!(input,
//        alt!(
//            not_start_mustache  |
//            alt_parsed          |
//            start_mustache
//        )
//    );
//    IResult::Done(i, text)
//}

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
    let (i, (_, _, name, _, _)) = try_parse!(input,
        tuple!(
            tag!("{{"),
            opt!(multispace),
            map_res!(
                take_until!("}}"),
                str::from_utf8
            ),
            opt!(multispace),
            tag!("}}")
        )
    );
    IResult::Done(i, Parsed::Mustache(name.to_owned()))
}

