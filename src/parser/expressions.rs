use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, space, multispace};

use container::expression::*;

use parser::literals::literal;


// ---------------------------------------------------------------------------

named!(pub full_expression<&[u8], FullExpression>, do_parse!(
    expr: expression                        >>
    filters: filter_agg                     >>
    ( FullExpression::new(expr, filters) )
));

// ---------------------------------------------------------------------------

named!(pub filter_agg<&[u8], Vec<FilterItem> >, many0!(do_parse!(
    many0!(multispace)  >>
    f: filter           >>
    (f)
)));


named!(pub filter<&[u8], FilterItem>, do_parse!(
    char!('|')          >>
    opt!(multispace)    >>
    id: identifier      >>
    ( FilterItem::Simple(id) )
));

// ---------------------------------------------------------------------------

named!(pub identifier<&[u8], String>, do_parse!(
    res: map_res!(
        recognize!(do_parse!(alpha >> many0!(alt!(tag!("_") | alphanumeric)) >> (()) )),
        str::from_utf8
    ) >>
    ( res.to_string() )
));

// ---------------------------------------------------------------------------

named!(pub expression<&[u8], DisjExpr>, do_parse!(
    a:  conj                                                >>
    b:  many0!(do_parse!(op_disj_bin >> c: conj >> (c)))    >>
    ( {
        let mut list = b;
        list.insert(0, a);
        DisjExpr { list }
    } )
));

named!(pub conj<&[u8], ConjExpr>, do_parse!(
    a:  cmp                                                 >>
    b:  many0!(do_parse!(op_conj_bin >> c: cmp >> (c)))     >>
    ( {
        let mut list = b;
        list.insert(0, a);
        ConjExpr { list }
    } )
));

named!(pub cmp<&[u8], CmpExpr>, do_parse!(
    a:  do_parse!(op: value!(CmpOp::Eq) >> t: sum >> ( CmpItem(op, t) ))    >>
    b:  many0!(do_parse!(op: op_cmp_bin >> t: sum >> ( CmpItem(op, t) )) )  >>
    ( {
        let mut list = b;
        list.insert(0, a);
        CmpExpr { list }
    } )
));

named!(pub sum<&[u8], Expr>, do_parse!(
    a:  do_parse!(op: value!(SumOp::Add) >> t: mul >> ( ExprItem(op, t) ))  >>
    b:  many0!(do_parse!(op: op_sum_bin >> t: mul >> ( ExprItem(op, t) )))  >>
    ( {
        let mut list = b;
        list.insert(0, a);
        Expr { sum: list }
    } )
));

named!(pub mul<&[u8], Term>, do_parse!(
    a:  do_parse!(op: value!(MulOp::Mul) >> t: factor >> ( TermItem(op, t) ))   >>
    b:  many0!(do_parse!(op: op_mul_bin >> t: factor >> ( TermItem(op, t) )))   >>
    ( {
        let mut list = b;
        list.insert(0, a);
        Term { mul: list }
    } )
));

#[derive(Debug)]
enum RevFactor {
    Invocation(Vec<DisjExpr>),
    Index(DisjExpr),
    Attribute(String),
}

named!(factor<&[u8], Factor>, do_parse!(
    a:  simple_factor       >>
    b:  many0!(attr_chain)  >>
    ( {
        let res = b.into_iter().fold(a, |acc, rf| {
            trace!("::: acc, rf: {:?}, {:?}", acc, rf);
            match rf {
                RevFactor::Attribute(id) => Factor::Attribute(Attribute { id: id, on: box acc }),
                RevFactor::Index(expr) => Factor::Index(Index { index: box expr, on: box acc }),
                RevFactor::Invocation(args) => Factor::Invocation(Invocation { args: args, on: box acc }),
            }
        } );
        trace!("::: factor: {:?}", res);
        res
    })
));

named!(attr_chain<&[u8], RevFactor>,
    do_parse!(
        many0!(multispace) >>
        rf: alt!(
            do_parse!(char!('.') >> many0!(multispace) >> id: identifier >>
                (RevFactor::Attribute(id))
            ) |
            do_parse!(char!('(') >> many0!(multispace) >> args: expr_list >> many0!(multispace) >> char!(')') >>
                (RevFactor::Invocation(args))
            ) |
            do_parse!(char!('[') >> index: expression >> char!(']') >>
                (RevFactor::Index(index))
            )
        ) >> ({
            trace!("::: RevFactor: {:?}", rf);
            rf
        })
    )
);

named!(simple_factor<&[u8], Factor>, alt!(literal | variable | subexpression) );

named!(subexpression<&[u8], Factor>, do_parse!(
    char!('(') >> many0!(multispace) >> e: expression >> many0!(multispace) >> char!(')') >>
    (Factor::Subexpression(e))
));

named!(variable<&[u8], Factor>, do_parse!(
    id: identifier >>
    inv: opt!(complete!(do_parse!(
        many0!(multispace) >> char!('(') >> many0!(multispace) >>
        e: expr_list >> many0!(multispace) >> char!(')') >>
        (e)
    ))) >>
    ({
        let var = Factor::Variable(id);
        let v = match inv {
            Some(args) => Invocation { on: Box::new(var), args: args } .into(),
            None => var
        };
        trace!("::: v: {:?}", v);
        v
    })
));

named!(expr_list<&[u8], Vec<DisjExpr> >, do_parse!(
    lst: opt!(do_parse!(
        a: expression                                               >>
        b: many0!(do_parse!(expr_sep >> e: expression >> (e) ))     >>
        ({
            let mut lst = b;
            lst.insert(0, a);
            lst
        })
    )) >>
    ({
        trace!("::: expr_list: {:?}", lst);
        lst.unwrap_or_else(Vec::new)
    })
));



named!(expr_sep<&[u8], ()>, do_parse!(
    many0!(multispace) >> char!(',') >> many0!(multispace) >>
    (())
));


named!(op_disj_bin<&[u8], ()>, do_parse!(
    many0!(multispace) >> o: tag!("or") >> many0!(multispace) >>
    ({
        trace!(":::: or");
        match o {
            b"or" => (),
            _ => unreachable!()
        }
    })
));

named!(op_conj_bin<&[u8], ()>, do_parse!(
    many0!(multispace) >> o: tag!("and") >> many0!(multispace) >>
    ({
        trace!(":::: and");
        match o {
            b"and" => (),
            _ => unreachable!()
        }
    })
));

named!(op_cmp_bin<&[u8], CmpOp>, do_parse!(
    many0!(multispace) >>
    o: alt!(tag!("<=") | tag!("<") | tag!("==") | tag!("!=") | tag!("in") | tag!("not in") | tag!(">=") | tag!(">")) >>
    many0!(multispace) >>
    ({
        trace!(":::: cmp {:?}", o);
        match o {
            b"<"        => CmpOp::Lt,
            b"<="       => CmpOp::Lte,
            b"=="       => CmpOp::Eq,
            b"!="       => CmpOp::Neq,
            b"in"       => CmpOp::In,
            b"not in"   => CmpOp::Nin,
            b">="       => CmpOp::Gte,
            b">"        => CmpOp::Gt,
            _ => unreachable!()
        }
    })
));

named!(op_sum_bin<&[u8], SumOp>, do_parse!(
    many0!(multispace) >> o: alt!(tag!("+") | tag!("-")) >> many0!(multispace) >>
    ({
        match o {
            b"+" => SumOp::Add,
            b"-" => SumOp::Sub,
            _ => unreachable!()
        }
    })
));

named!(op_mul_bin<&[u8], MulOp>, do_parse!(
    many0!(multispace) >> o: alt!(tag!("*") | tag!("/")) >> many0!(multispace) >>
    ({
        match o {
            b"*" => MulOp::Mul,
            b"/" => MulOp::Div,
            _ => unreachable!()
        }
    })
));

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
    fn format() {
        use container::expression::FilterItem::Simple;
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"| e"));
        assert_eq!(Done(&b""[..], Simple("e".into())), super::filter(b"|  e"));
        assert!(super::filter(b"| 1wrong").is_err());
        assert!(super::filter(b"| _wrong").is_err());
    }
}













/*
http://codinghighway.com/2015/05/25/down-with-redundant-parenthesis/

expr -> term expr_r

expr_r -> '+' term expr_r
        | '-' term expr_r
        | e

term -> factor term_r

term_r -> '*' factor term_r
        | '/' factor term_r
        | e

factor -> number
        | '(' expr ')'
*/

/*

5 + 6

expr [5 + 6
expr [term [5 + 6
expr [term [factor [5 + 6
expr [term [factor [number[5 + 6
expr [term [factor [number[5] + 6
expr [term [factor [number[5]] + 6
expr [term [factor [number[5]] expr_r [ + 6
expr [term [factor [number[5]] expr_r ['+' 6
expr [term [factor [number[5]] expr_r ['+' term [6
expr [term [factor [number[5]] expr_r ['+' term [factor [6
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [e
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [e]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [e]]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [e]]]
expr [term [factor [number[5]] expr_r ['+' term [factor [number[6]] term_r [e]] expr_r [e]]]]

expr
term
factor   expr_r
number   '+' term              expr_r
5            factor   term_r   e
             number   e
             6


5        '+' 6


5 + 6

expr [
    term [
        factor [
            number[5]
        ]
        expr_r [
            '+'
            term [
                factor [
                    number[6]
                ]
                term_r[e]
            ]
            expr_r[e]
        ]
    ]
]


expr:
    term:
        factor:
            number:
                5
        expr_r:
            '+'
            term:
                factor:
                    number:
                        6
                term_r:
                    e
            expr_r:
                e
*/
