use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, space, multispace};

use container::expression::*;
use container::parsed::*;

use super::block_level::nodes;


named!(pub statement<&[u8], ParsedNode>,
    alt!( if_block | for_block | raw_block | block_block | extends_block | include_block )
);

named!(pub stmt_edge<&[u8], ParsedNode>, do_parse!(
    tag!("{%") >> (ParsedNode::Text("".into()))
));

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_raw<&[u8], SimpleStatement>, stmt_simple!("raw"));
named!(pub stmt_endraw<&[u8], SimpleStatement>, stmt_simple!("endraw"));

fn raw_block(input: &[u8]) -> IResult<&[u8], ParsedNode> {
    fn is_end(input: &[u8]) -> bool {
        stmt_endraw(input).is_done()
    }
    do_parse!(
        input,
        b: stmt_raw >>
        txt: map_res!(
            take_till_slc!(is_end),
            str::from_utf8
        ) >>
        e: stmt_endraw >>
        (ParsedRawStatement { begin: b, text: txt.into(), end: e }.into())
    )
}

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_for<&[u8], ExprStatement>, stmt_expr!("for"));
named!(pub stmt_endfor<&[u8], SimpleStatement>, stmt_simple!("endfor"));

named!(pub for_block<&[u8], ParsedNode>, do_parse!( s: for_statement >> (s.into()) ));

pub fn for_statement(input: &[u8]) -> IResult<&[u8], ParsedForStatement> {
    fn finish(begin: ExprStatement, block: ParsedNodes, end: SimpleStatement) -> Option<(ParsedForStatement)> {
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
                let expression = FullExpression {
                    filters: begin.expression.filters,
                    expr: DisjExpr { list: vec![
                        ConjExpr { list: vec![
                            CmpExpr { list: vec![ right ] }
                        ]}
                    ]},
                };
                let begin = ExprStatement { expression, ..begin };
                Some(ParsedForStatement { begin, block, key_var: None, value_var, end })
            },
            _ => None
        }
    }

    let (i, (b, t, e)) = try_parse!(input, tuple!( stmt_for, nodes, stmt_endfor ) );

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

named!(pub if_block<&[u8], ParsedNode>, do_parse!( s: if_statement >> ( s.into() ) ));

pub fn if_statement(input: &[u8]) -> IResult<&[u8], ParsedIfStatement> {

    named!(if_<&[u8], ParsedIfBranch>, do_parse!(
        begin: stmt_if >> block: nodes >> (ParsedIfBranch { begin, block })
    ));
    named!(elif<&[u8], ParsedIfBranch>, do_parse!(
        begin: stmt_elif >> block: nodes >> (ParsedIfBranch { begin, block })
    ));
    named!(else_<&[u8], ParsedElseBranch>, do_parse!(
        begin: stmt_else >> block: nodes >> (ParsedElseBranch { begin, block })
    ));

    do_parse!( input,
        if_branches: do_parse!(
            ifs: if_ >>
            elifs: many0!(elif) >>
            ({
                let mut list = elifs;
                list.insert(0, ifs);
                list
            })
        ) >>
        else_branch: opt!(else_) >>
        end: stmt_endif >>
        (ParsedIfStatement { if_branches, else_branch, end })
    )
}


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_block<&[u8], NamedStatement>, stmt_named!("block"));
named!(pub stmt_endblock<&[u8], SimpleStatement>, stmt_simple!("endblock"));

named!(pub block_block<&[u8], ParsedNode>, do_parse!( s: block_statement >> (s.into()) ));

named!(pub block_statement<&[u8], ParsedBlockStatement>, do_parse!(
    begin: stmt_block >>
    block: nodes >>
    end: stmt_endblock >>
    (ParsedBlockStatement { begin, block, end })
));


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_extends<&[u8], ExprStatement>, stmt_expr!("extends"));

named!(pub extends_block<&[u8], ParsedNode>, do_parse!(
    s: stmt_extends >> (ParsedNode::Extends(s))
));


// ---------------------------------------------------------------------------

named!(pub stmt_include<&[u8], ExprStatement>, stmt_expr!("include"));

named!(pub include_block<&[u8], ParsedNode>, do_parse!(
    s: stmt_include >> (ParsedNode::Include(s))
));


// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #![allow(clippy::used_underscore_binding)]

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
