use std::str;
#[allow(unused_imports)]
use nom::{IResult, Err as NomErr, ErrorKind, alpha, alphanumeric, space, multispace};

use ::template::{
    Parsed, Statement, FullExpression, Template, Factor,
    IfStatement, IfBranch, ForStatement,
    DisjExpr, ConjExpr, CmpExpr, Expr, Term,
    DisjOp, ConjOp, CmpOp,
    DisjItem, ConjItem,
};

use super::block_level::{inner};


named!(pub statement<&[u8], Parsed>,
    alt!( raw_block | if_block | for_block )
);

pub fn stmt_edge(input: &[u8]) -> IResult<&[u8], Parsed> {
    let (_, _) = try_parse!(input,
        alt!(
            stmt_raw |
            stmt_if | stmt_elif | stmt_else | stmt_endif |
            stmt_for | stmt_endfor
        )
    );
    IResult::Done(input, Parsed::Text("".into()))
}


// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_raw<&[u8], Statement>, stmt!("raw"));
named!(pub stmt_endraw<&[u8], Statement>, stmt!("endraw"));

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

named!(pub for_block<&[u8], Parsed>, chain!( s: for_statement, || s.into() ));

pub fn for_statement(input: &[u8]) -> IResult<&[u8], ForStatement> {
    fn finish(begin: Statement, inner: Template, end: Statement) -> Option<(ForStatement)> {
        match begin.expression {
            None => None,
            Some(full_expr) => {
                let mut expr: DisjExpr = full_expr.expr;
                if expr.list.len() != 1 {return None}

                let mut expr: ConjExpr = expr.list.remove(0).1;
                if expr.list.len() != 1 {return None}

                let mut expr: CmpExpr = expr.list.remove(0).1;
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
                            filters: full_expr.filters,
                            expr: DisjExpr{list: vec![
                                DisjItem(DisjOp::Or, ConjExpr{list: vec![
                                    ConjItem(ConjOp::And, CmpExpr{list: vec![ right ]})
                                ]}) ]},
                        };
                        let new_begin = Statement {
                            strip_left: begin.strip_left,
                            strip_right: begin.strip_right,
                            expression: Some(expr)
                        };
                        Some(ForStatement{
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
        }
    }

    let (i, (b, t, e)) = try_parse!(input,
        tuple!(
            stmt_for, inner, stmt_endfor
        )
    );

    match finish(b, Template {parsed: t}, e) {
        Some(result) => IResult::Done(i, result),
        None => IResult::Error(NomErr::Code(ErrorKind::Custom(1331))),
    }
}

// --------------------------------------------------------------------------------------------------------------------

named!(pub stmt_if<&[u8], Statement>, stmt!("if"));
named!(pub stmt_elif<&[u8], Statement>, stmt!("elif"));
named!(pub stmt_else<&[u8], Statement>, stmt!("else"));
named!(pub stmt_endif<&[u8], Statement>, stmt!("endif"));

named!(pub if_block<&[u8], Parsed>, chain!( s: if_statement, || s.into() ));

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
        use ::template::{Statement, IfStatement, IfBranch, FullExpression, Template,
            DisjExpr, DisjOp, DisjItem, ConjExpr, ConjOp, ConjItem,
            CmpExpr, CmpOp, CmpItem, Expr, ExprItem, SumOp, Term, TermItem, MulOp, Factor};

        let sample = |r| IfStatement {
            if_branches: vec![
                IfBranch {
                    begin: Statement {
                        strip_left: false,
                        strip_right: r,
                        expression: Some(FullExpression {
                            expr: DisjExpr {
                                list: vec![DisjItem(DisjOp::Or, ConjExpr {
                                    list: vec![ConjItem(ConjOp::And, CmpExpr {
                                        list: vec![CmpItem(CmpOp::Eq, Expr {
                                            sum: vec![ExprItem(SumOp::Add, Term {
                                                mul: vec![TermItem(MulOp::Mul, Factor::Variable("True".into()) )],
                                            })],
                                        })],
                                    })],
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
    fn for_() {
        // TODO weird test
//        use ::template::Parsed::Text;
//        assert_eq!(Done(&b""[..], Text("{{ i }}".into())), super::for_block(b"{% for %}{{ i }}{% endfor %}"));
    }
}
