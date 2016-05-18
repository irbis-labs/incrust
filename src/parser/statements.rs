use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, eof, space, multispace};

use ::template::{Parsed, Statement, IfStatement, IfBranch, Template};

use super::block_level::{inner};


named!(pub statement<&[u8], Parsed>,
    alt!( raw_block | if_block )
);

pub fn stmt_edge(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (_, _) = try_parse!(input,
        alt!(
            stmt_raw |
            stmt_if | stmt_elif | stmt_else | stmt_endif
        )
    );
    IResult::Done(input, Parsed::Text("".into()))
}


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

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_for<&[u8], Statement>, stmt!("for"));
named!(pub stmt_endfor<&[u8], Statement>, stmt!("endfor"));

#[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
pub fn for_block(input: &[u8]) -> IResult<&[u8], Parsed> {
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

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_if<&[u8], Statement>, stmt!("if"));
named!(pub stmt_elif<&[u8], Statement>, stmt!("elif"));
named!(pub stmt_else<&[u8], Statement>, stmt!("else"));
named!(pub stmt_endif<&[u8], Statement>, stmt!("endif"));

named!(pub if_block<&[u8], Parsed>, chain!( s: if_statement, || s.into() ));

#[cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]
pub fn if_statement(input: &[u8]) -> IResult<&[u8], IfStatement> {

    named!(if_<&[u8], IfBranch>, chain!( s: stmt_if ~ b: inner, || IfBranch {begin: s, block: Template {parsed: b}} ));
    named!(elif<&[u8], IfBranch>, chain!( s: stmt_elif ~ b: inner, || IfBranch {begin: s, block: Template {parsed: b}} ));
    named!(else_<&[u8], IfBranch>, chain!( s: stmt_else ~ b: inner, || IfBranch {begin: s, block: Template {parsed: b}} ));

    let (i, (ifs, elses, end)) = try_parse!(input,
        tuple!(
            chain!(
                ifs: if_ ~
                mut elifs: many0!(elif),
                || { elifs.insert(0, ifs); elifs }
            ),
            opt!(else_),
            stmt_endif
        )
    );
    IResult::Done(i, IfStatement {
        if_branches: ifs,
        else_branch: elses,
        end: end,
    })
}


// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    use nom::IResult::Done;

    #[test]
    fn statement() {
        use ::template::Statement;
        let empty_statement = Statement::default();
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{%raw%}"));
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{% raw %}"));
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{%  raw  %}"));
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{%\traw\t%}"));
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{%\nraw\n%}"));
        assert_eq!(Done(&b""[..], empty_statement.clone()),  super::stmt_raw(b"{%\rraw\r%}"));

        let strip_statement = Statement { strip_left: true, strip_right: true, expression: None };
        assert_eq!(Done(&b""[..], strip_statement.clone()),  super::stmt_raw(b"{%-raw-%}"));
        assert_eq!(Done(&b""[..], strip_statement.clone()),  super::stmt_raw(b"{%- raw -%}"));
    }

    #[test]
    fn raw() {
        use ::template::Parsed::Text;
        assert_eq!(Done(&b""[..], Text("{{ raw }}".into())), super::raw_block(b"{% raw %}{{ raw }}{% endraw %}"));
        assert_eq!(Done(&b""[..], Text("{% if %}".into())),  super::raw_block(b"{% raw %}{% if %}{% endraw %}"));
        assert_eq!(Done(&b""[..], Text("{% if %}".into())),  super::raw_block(b"{%  raw %}{% if %}{%  endraw %}"));
    }

    #[test]
    fn if_() {
        use ::template::Parsed::Text;
        use ::template::{Statement, IfStatement, IfBranch, FullExpression, Expr, ExprItem, SumOp, Term, TermItem, MulOp, Factor, Template};

        let sample = |r| IfStatement {
            if_branches: vec![
                IfBranch {
                    begin: Statement {
                        strip_left: false,
                        strip_right: r,
                        expression: Some(FullExpression {
                            expr: Expr {
                                sum: vec![ExprItem(SumOp::Add, Term {
                                    mul: vec![TermItem(MulOp::Mul, Factor::Variable("True".into()) )],
                                })],
                            },
                            filters: Vec::new(),
                        })
                    },
                    block: Template {
                        parsed: vec![Text("_".into())]
                    }
                }
            ],
            else_branch: None,
            end: Statement::default(),
        };

        assert_eq!(Done(&b""[..], sample(false)), super::if_statement(b"{% if True %}_{% endif %}"));
        assert_eq!(Done(&b""[..], sample(true)),  super::if_statement(b"{% if True-%}_{% endif %}"));
        assert_eq!(Done(&b""[..], sample(true)),  super::if_statement(b"{% if True -%}_{% endif %}"));
    }

    #[test]
    fn foreach() {
        use ::template::Parsed::Text;
        assert_eq!(Done(&b""[..], Text("{{ i }}".into())), super::for_block(b"{% for %}{{ i }}{% endfor %}"));
    }
}
