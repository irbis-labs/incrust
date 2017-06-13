use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, space, multispace};

use container::expression::Mustache;
use container::parsed::ParsedNode;

use parser::statements::statement;
use parser::statements::stmt_edge;
use parser::expressions::full_expression;


named!(pub text<&[u8], Vec<ParsedNode> >, terminated!( nodes, eof!() ) );


pub fn nodes(input: &[u8]) -> IResult<&[u8], Vec<ParsedNode> > {
    let (i, list) = try_parse!(input, many0!(node) );
    IResult::Done(i, list)
}



named!(node<&[u8], ParsedNode>,
    alt!(
        plain_text          |
        comment             |
        mustache            |
        statement
    )
);


fn comment(input: &[u8]) -> IResult<&[u8], ParsedNode> {
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
    IResult::Done(i, ParsedNode::Comment(comment.to_owned()))
}


fn mustache(input: &[u8]) -> IResult<&[u8], ParsedNode> {
    do_parse!(input,
            tag!("{{")          >>
            many0!(multispace)  >>
        fe: full_expression     >>
            many0!(multispace)  >>
            tag!("}}")          >>
        ( Mustache::new(fe).into() )
    )
}


fn plain_text(input: &[u8]) -> IResult<&[u8], ParsedNode> {
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
        do_parse!(
            v: many1!(
                alt!( map_res!( is_not!("{"), str::from_utf8 ) | try_brace )
            ) >>
            ( v.join("") )
        )
    );
    IResult::Done(i, ParsedNode::Text(text))
}



// ---------------------------------------------------------------------------


//#[cfg(test)]
//mod tests {
//    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
//
//    use nom::IResult::Done;
//
//}
