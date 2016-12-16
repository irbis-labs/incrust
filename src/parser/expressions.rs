use std::str;
#[allow(unused_imports)]
use nom::{IResult, alpha, alphanumeric, space, multispace};

use container::expression::*;

use parser::literals::literal;


// ---------------------------------------------------------------------------

pub fn full_expression(input: &[u8]) -> IResult<&[u8], FullExpression> {
    let (i, (expr, filters)) = try_parse!(input, tuple!( expression, filter_agg ) );
    IResult::Done(i, FullExpression::new(expr, filters))
}

named!(pub filter_agg<&[u8], Vec<FilterItem> >,
    many0!(chain!(
        many0!(multispace) ~
        f: filter,
        || f
    ))
);


pub fn filter(input: &[u8]) -> IResult<&[u8], FilterItem> {
    let (i, (_, _, id)) = try_parse!(input,
        tuple!(
            tag!("|"),
            opt!(multispace),
            identifier
        )
    );
    IResult::Done(i, FilterItem::Simple(id))
}

// ---------------------------------------------------------------------------

pub fn identifier(input: &[u8]) -> IResult<&[u8], String> {
    let (i, id) = try_parse!(input,
        chain!(
            start: map_res!(alpha, str::from_utf8)~
            rest: many0!(map_res!(alt!(tag!("_") | alphanumeric), str::from_utf8)),
            || {
                rest.into_iter().fold(start.to_string(), |mut acc, slice| {
                    acc.push_str(slice);
                    acc
                })
            }
        )
    );
    trace!("::: identifier {:?}", id);
    IResult::Done(i, id)
}

// ---------------------------------------------------------------------------

pub fn expression(input: &[u8]) -> IResult<&[u8], DisjExpr> {
    let (i, lst) = try_parse!(input, chain!(
        a: tuple!(value!(()), conj) ~
        mut b: many0!(tuple!(op_disj_bin, conj)) ,
        || { b.insert(0, a); b }
    ));
    IResult::Done(i, DisjExpr { list: lst.into_iter().map(|(_, t)| t ).collect() })
}

pub fn conj(input: &[u8]) -> IResult<&[u8], ConjExpr> {
    let (i, lst) = try_parse!(input, chain!(
        a: tuple!(value!(()), cmp) ~
        mut b: many0!(tuple!(op_conj_bin, cmp)) ,
        || { b.insert(0, a); b }
    ));
    IResult::Done(i, ConjExpr { list: lst.into_iter().map(|(_, t)| t ).collect() })
}

pub fn cmp(input: &[u8]) -> IResult<&[u8], CmpExpr> {
    let (i, lst) = try_parse!(input, chain!(
        a: tuple!(value!(CmpOp::Eq), sum) ~
        mut b: many0!(tuple!(op_cmp_bin, sum)) ,
        || { b.insert(0, a); b }
    ));
    IResult::Done(i, CmpExpr { list: lst.into_iter().map(|(op, t)| CmpItem(op, t) ).collect() })
}

pub fn sum(input: &[u8]) -> IResult<&[u8], Expr> {
    let (i, sum) = try_parse!(input, chain!(
        a: tuple!(value!(SumOp::Add), mul) ~
        mut b: many0!(tuple!(op_sum_bin, mul)) ,
        || { b.insert(0, a); b }
    ));
    IResult::Done(i, Expr { sum: sum.into_iter().map(|(op, t)| ExprItem(op, t) ).collect() })
}

fn mul(input: &[u8]) -> IResult<&[u8], Term> {
    let (i, mul) = try_parse!(input, chain!(
        a: tuple!(value!(MulOp::Mul), factor) ~
        mut b: many0!(tuple!(op_mul_bin, factor)) ,
        || { b.insert(0, a); b }
    ));
    IResult::Done(i, Term{mul: mul.into_iter().map(|(op, f)| TermItem(op, f) ).collect()} )
}

fn factor(input: &[u8]) -> IResult<&[u8], Factor> {
    let (i, res) = try_parse!(input, chain!(
        a: simple_factor ~
        b: many0!(chain!(many0!(multispace) ~ char!('.') ~ many0!(multispace) ~ id: variable, || id)),
        || b.into_iter().fold(a, |acc, arg| {
            trace!("::: acc, arg: {:?}, {:?}", acc, arg);
            match arg {
                Factor::Variable(id) => Factor::Attribute(Attribute{ id: id, on: Box::new(acc) }),
                Factor::Invocation(inv) => Factor::Invocation(inv),
                _ => unreachable!()
            }
        })
    ) );
    trace!("::: factor: {:?}", res);
    IResult::Done(i, res)
}

fn simple_factor(input: &[u8]) -> IResult<&[u8], Factor> {
    let (i, f) = try_parse!(input, alt!(literal | variable | subexpression) );
    IResult::Done(i, f)
}

fn subexpression(input: &[u8]) -> IResult<&[u8], Factor> {
    let (i, (_, _, e, _, _)) = try_parse!(input, tuple!(
        char!('('), many0!(multispace), expression, many0!(multispace), char!(')')
    ));
    IResult::Done(i, Factor::Subexpression(e))
}


fn variable(input: &[u8]) -> IResult<&[u8], Factor> {
    let (i, (id, inv)) = try_parse!(input, tuple!(
        identifier,
        opt!(complete!(tuple!(
            many0!(multispace), char!('('), many0!(multispace), expr_list, many0!(multispace), char!(')')
        )))
    ));
    let var = Factor::Variable(id);
    let v = match inv {
        Some((_, _, _, args, _, _)) => Invocation { on: Box::new(var), args: args } .into(),
        None => var
    };
    trace!("::: v: {:?}", v);
    IResult::Done(i, v)
}

fn expr_list(input: &[u8]) -> IResult<&[u8], Vec<DisjExpr> > {
    let (i, lst) = try_parse!(input, opt!(chain!(
        a: expression ~
        b: many0!(tuple!(expr_sep, expression)),
        || {
            let mut lst = Vec::with_capacity(b.len() + 1);
            lst.push(a);
            lst.extend(b.into_iter().map(|(_, e)| e));
            lst
        }
    )));
    trace!("::: expr_list: {:?}", lst);
    IResult::Done(i, lst.unwrap_or_else(Vec::new))
}



fn expr_sep(input: &[u8]) -> IResult<&[u8], ()> {
    let (i, _) = try_parse!(input, tuple!(many0!(multispace), char!(','), many0!(multispace)) );
    IResult::Done(i, ())
}


fn op_disj_bin(input: &[u8]) -> IResult<&[u8], ()> {
    let (i, (_, o, _)) = try_parse!(input, tuple!(many0!(multispace), alt!(tag!("or")), many0!(multispace)) );

    trace!(":::: or");
    IResult::Done(i, match o {
        b"or" => (),
        _ => unreachable!()
    })
}

fn op_conj_bin(input: &[u8]) -> IResult<&[u8], ()> {
    let (i, (_, o, _)) = try_parse!(input, tuple!(many0!(multispace), alt!(tag!("and")), many0!(multispace)) );

    trace!(":::: and");
    IResult::Done(i, match o {
        b"and" => (),
        _ => unreachable!()
    })
}

fn op_cmp_bin(input: &[u8]) -> IResult<&[u8], CmpOp> {
    let (i, (_, o, _)) = try_parse!(input, tuple!(
        many0!(multispace),
        alt!(tag!("<=") | tag!("<") | tag!("==") | tag!("!=") | tag!("in") | tag!("not in") | tag!(">=") | tag!(">")),
        many0!(multispace)
    ));
    trace!(":::: cmp {:?}", o);
    IResult::Done(i, match o {
        b"<"        => CmpOp::Lt,
        b"<="       => CmpOp::Lte,
        b"=="       => CmpOp::Eq,
        b"!="       => CmpOp::Neq,
        b"in"       => CmpOp::In,
        b"not in"   => CmpOp::Nin,
        b">="       => CmpOp::Gte,
        b">"        => CmpOp::Gt,
        _ => unreachable!()
    })
}

fn op_sum_bin(input: &[u8]) -> IResult<&[u8], SumOp> {
    let (i, (_, o, _)) = try_parse!(input, tuple!(many0!(multispace), alt!(tag!("+") | tag!("-")), many0!(multispace)) );

    IResult::Done(i, match o {
        b"+" => SumOp::Add,
        b"-" => SumOp::Sub,
        _ => unreachable!()
    })
}

fn op_mul_bin(input: &[u8]) -> IResult<&[u8], MulOp> {
    let (i, (_, o, _)) = try_parse!(input, tuple!(many0!(multispace), alt!(tag!("*") | tag!("/")), many0!(multispace)) );

    IResult::Done(i, match o {
        b"*" => MulOp::Mul,
        b"/" => MulOp::Div,
        _ => unreachable!()
    })
}

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
