use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, space, multispace};

use container::expression::*;
use container::parsed::*;

use super::block_level::nodes;


named!(pub statement<&[u8], ParsedNode>,
    alt!( if_block | for_block | raw_block | block_block | extends_block )
);

pub fn stmt_edge(input: &[u8]) -> IResult<&[u8], ParsedNode> {
    let (_, _) = try_parse!(input, tag!("{%"));
    IResult::Done(input, ParsedNode::Text("".into()))
}


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_raw<&[u8], SimpleStatement>, stmt_simple!("raw"));
named!(pub stmt_endraw<&[u8], SimpleStatement>, stmt_simple!("endraw"));

fn raw_block(input: &[u8]) -> IResult<&[u8], ParsedNode> {
    #[cfg_attr(feature = "clippy", allow(needless_lifetimes))]
    fn is_end<'a>(input: &'a [u8]) -> bool {
        let b = || -> IResult<&'a [u8], ()> {
            let _ = try_parse!(input, stmt_endraw );
            IResult::Done(input, ())
        };
        b().is_done()
    }

    let (i, (b, txt, e)) = try_parse!(input,
        tuple!(
            stmt_raw,
            map_res!(
                take_till_slc!(is_end),
                str::from_utf8
            ),
            stmt_endraw
        )
    );
    IResult::Done(i, ParsedRawStatement { begin: b, text: txt.into(), end: e }.into())
}

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_for<&[u8], ExprStatement>, stmt_expr!("for"));
named!(pub stmt_endfor<&[u8], SimpleStatement>, stmt_simple!("endfor"));

named!(pub for_block<&[u8], ParsedNode>, chain!( s: for_statement, || s.into() ));

pub fn for_statement(input: &[u8]) -> IResult<&[u8], ParsedForStatement> {
    fn finish(begin: ExprStatement, inner: ParsedNodes, end: SimpleStatement) -> Option<(ParsedForStatement)> {
        let mut expr: DisjExpr = begin.expression.expr;
        if expr.list.len() != 1 {return None}

        let mut expr: ConjExpr = expr.list.remove(0);
        if expr.list.len() != 1 {return None}

        let mut expr: CmpExpr = expr.list.remove(0);
        if expr.list.len() != 2 {return None}

        let left = expr.list.remove(0);
        let right = expr.list.remove(0);

        match right.0 {
            CmpOp::In => {},
            _ => return None,
        }

        let mut expr: Expr = left.1;
        if expr.sum.len() != 1 {return None}

        let mut expr: Term = expr.sum.remove(0).1;
        if expr.mul.len() != 1 {return None}

        let left = expr.mul.remove(0).1;

        match left {
            Factor::Variable(value_var) => {
                let expr = FullExpression {
                    filters: begin.expression.filters,
                    expr: DisjExpr { list: vec![
                        ConjExpr { list: vec![
                            CmpExpr { list: vec![ right ] }
                        ]}
                    ]},
                };
                let new_begin = ExprStatement {
                    expression: expr,
                    .. begin
                };
                Some(ParsedForStatement {
                    begin: new_begin,
                    block: inner,
                    key_var: None,
                    value_var: value_var,
                    end: end,
                })
            },
            _ => None
        }
    }

    let (i, (b, t, e)) = try_parse!(input,
        tuple!(
            stmt_for, nodes, stmt_endfor
        )
    );

    match finish(b, t, e) {
        Some(result) => IResult::Done(i, result),
        None => IResult::Error(NomErr::Code(ErrorKind::Custom(1331))),
    }
}

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_if<&[u8], ExprStatement>, stmt_expr!("if"));
named!(pub stmt_elif<&[u8], ExprStatement>, stmt_expr!("elif"));
named!(pub stmt_else<&[u8], SimpleStatement>, stmt_simple!("else"));
named!(pub stmt_endif<&[u8], SimpleStatement>, stmt_simple!("endif"));

named!(pub if_block<&[u8], ParsedNode>, chain!( s: if_statement, || s.into() ));

pub fn if_statement(input: &[u8]) -> IResult<&[u8], ParsedIfStatement> {

    named!(if_<&[u8], ParsedIfBranch>, chain!( s: stmt_if ~ b: nodes, || ParsedIfBranch { begin: s, block: b } ));
    named!(elif<&[u8], ParsedIfBranch>, chain!( s: stmt_elif ~ b: nodes, || ParsedIfBranch { begin: s, block: b } ));
    named!(else_<&[u8], ParsedElseBranch>, chain!( s: stmt_else ~ b: nodes, || ParsedElseBranch { begin: s, block: b } ));

    let (i, (ifs, els, end)) = try_parse!(input,
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
    IResult::Done(i, ParsedIfStatement {
        if_branches: ifs,
        else_branch: els,
        end: end,
    })
}


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_block<&[u8], NamedStatement>, stmt_named!("block"));
named!(pub stmt_endblock<&[u8], SimpleStatement>, stmt_simple!("endblock"));

named!(pub block_block<&[u8], ParsedNode>, chain!( s: block_statement, || s.into() ));

pub fn block_statement(input: &[u8]) -> IResult<&[u8], ParsedBlockStatement> {

    let (i, (b, t, e)) = try_parse!(input,
        tuple!(
            stmt_block, nodes, stmt_endblock
        )
    );

    IResult::Done(i, ParsedBlockStatement {
        begin: b,
        block: t,
        end: e,
    })
}


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_extends<&[u8], ExprStatement>, stmt_expr!("extends"));

pub fn extends_block(input: &[u8]) -> IResult<&[u8], ParsedNode> {

    let (i, s) = try_parse!(input, stmt_extends);

    IResult::Done(i, ParsedNode::Extends(s))
}


// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    use nom::IResult::Done;
    use container::expression::*;
    use container::parsed::*;
    use container::parsed::ParsedNode::*;

    #[test]
    fn statement() {
        use super::stmt_raw;

        let empty_statement = SimpleStatement::default();
        let strip_statement = SimpleStatement { strip_left: true, strip_right: true };

        let expected = |statement: &SimpleStatement| Done(&b""[..], statement.clone());


        assert_eq!(expected(&empty_statement),  stmt_raw(b"{%raw%}"));
        assert_eq!(expected(&empty_statement),  stmt_raw(b"{% raw %}"));
        assert_eq!(expected(&empty_statement),  stmt_raw(b"{%  raw  %}"));
        assert_eq!(expected(&empty_statement),  stmt_raw(b"{%\traw\t%}"));
        assert_eq!(expected(&empty_statement),  stmt_raw(b"{%\nraw\n%}"));
        assert_eq!(expected(&empty_statement),  stmt_raw(b"{%\rraw\r%}"));

        assert_eq!(expected(&strip_statement),  stmt_raw(b"{%-raw-%}"));
        assert_eq!(expected(&strip_statement),  stmt_raw(b"{%- raw -%}"));
    }

    #[test]
    fn raw() {
        use super::raw_block;

        let expected = |txt| {
            Done(&b""[..], Raw(
                ParsedRawStatement {
                    begin: SimpleStatement { strip_left: false, strip_right: false },
                    text: String::from(txt),
                    end: SimpleStatement { strip_left: false, strip_right: false },
                }
            ))
        };

        assert_eq!(expected("{{ raw }}"), raw_block(b"{% raw %}{{ raw }}{% endraw %}"));
        assert_eq!(expected("{% if %}"),  raw_block(b"{% raw %}{% if %}{% endraw %}"));
        assert_eq!(expected("{% if %}"),  raw_block(b"{%  raw %}{% if %}{%  endraw %}"));
    }

    #[test]
    fn if_() {
        let sample = |strip_right| ParsedIfStatement {
            if_branches: vec![
                ParsedIfBranch {
                    begin: ExprStatement {
                        strip_left: false,
                        strip_right: strip_right,
                        expression: FullExpression {
                            expr: DisjExpr {
                                list: vec![ConjExpr {
                                    list: vec![CmpExpr {
                                        list: vec![CmpItem(CmpOp::Eq, Expr {
                                            sum: vec![ExprItem(SumOp::Add, Term {
                                                mul: vec![TermItem(MulOp::Mul, Factor::Variable("True".into()) )],
                                            })],
                                        })],
                                    }],
                                }],
                            },
                            filters: Vec::new(),
                        }
                    },
                    block: vec![Text("_".into())]
                }
            ],
            else_branch: None,
            end: SimpleStatement::default(),
        };

        assert_eq!(Done(&b""[..], sample(false)), super::if_statement(b"{% if True %}_{% endif %}"));
        assert_eq!(Done(&b""[..], sample(true)),  super::if_statement(b"{% if True-%}_{% endif %}"));
        assert_eq!(Done(&b""[..], sample(true)),  super::if_statement(b"{% if True -%}_{% endif %}"));
    }

    #[test]
    fn for_() {
        // TODO test for statement
    }
}
