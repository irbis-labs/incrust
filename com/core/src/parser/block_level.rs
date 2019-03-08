use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, Context, ErrorKind, alpha, alphanumeric, space, multispace};

use crate::container::expression::Mustache;
use crate::container::parsed::ParsedNode;

use crate::parser::statements::statement;
use crate::parser::statements::stmt_edge;
use crate::parser::expressions::full_expression;


named!(pub text<&[u8], Vec<ParsedNode> >, terminated!( nodes, eof!() ) );


pub fn nodes(input: &[u8]) -> IResult<&[u8], Vec<ParsedNode> > {
    let (i, list) = try_parse!(input, many0!(node) );
    Ok((i, list))
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
    Ok((i, ParsedNode::Comment(comment.to_owned())))
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
    fn try_brace<'a>(input: &'a [u8]) -> IResult<&[u8], &str> {
        let b = || -> IResult<&'a [u8], ()> {
            let _ = try_parse!(input, alt!( mustache | comment | stmt_edge ) );
            Ok((input, ()))
        };
        match b() {
            Err(_) => Ok((&input[1..], "{")),
            Ok((i, ())) => Err(NomErr::Error(Context::Code(i, ErrorKind::Custom(0)))),
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
    Ok((i, ParsedNode::Text(text)))
}



// ---------------------------------------------------------------------------


//#[cfg(test)]
//mod tests {
//    #![cfg_attr(feature = "cargo-clippy", allow(used_underscore_binding))]
//
//    use nom::IResult::Done;
//
//}
