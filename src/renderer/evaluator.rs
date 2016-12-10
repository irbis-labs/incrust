use std::borrow::Cow;

use abc::{EvalResult, EvalError};
use incrust::{Context, BType};
use template::{
    DisjExpr, ConjExpr, CmpExpr, Expr, Term,
    DisjOp, ConjOp, CmpOp, SumOp, MulOp,
    DisjItem, ConjItem, CmpItem, ExprItem, TermItem,
    Factor, Literal, Attribute, Invocation,
};


pub fn eval_expr<'a>(context: &'a Context, disj_expr: &'a DisjExpr) -> EvalResult<Cow<'a, BType>> {
    let mut itr = disj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&DisjItem(_, ref conj)) => {
            let mut acc = eval_conj(context, conj)?;
            for &DisjItem(ref op, ref conj) in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(acc) => match *op {
                        DisjOp::Or => match acc.to_bool() {
                            true => return Ok(Some(acc)),
                            false => eval_conj(context, conj)?,
                        },
                    } } }
            Ok(acc)
        } } }


pub fn eval_conj<'a>(context: &'a Context, conj_expr: &'a ConjExpr) -> EvalResult<Cow<'a, BType>> {
    let mut itr = conj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ConjItem(_, ref cmp)) => {
            let mut acc = eval_cmp(context, cmp)?;
            for &ConjItem(ref op, ref cmp) in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(acc) => match *op {
                        ConjOp::And => match acc.to_bool() {
                            true => eval_cmp(context, cmp)?,
                            false => return Ok(Some(acc))
                        },
                    } } }
            Ok(acc)
        } } }


#[allow(unused_variables)]
pub fn eval_cmp<'a>(context: &'a Context, cmp_expr: &'a CmpExpr) -> EvalResult<Cow<'a, BType>> {
    let mut itr = cmp_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&CmpItem(_, ref expr)) => {
            let acc = eval_sum(context, expr)?;
            for &CmpItem(ref op, ref expr) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_sum(context, expr)? {
                        None => return Ok(None),
                        Some(expr) => match *op {
                            CmpOp::Lt   => unimplemented!(),
                            CmpOp::Lte  => unimplemented!(),
                            CmpOp::Eq   => unimplemented!(),
                            CmpOp::Neq  => unimplemented!(),
                            CmpOp::In   => unimplemented!(),
                            CmpOp::Nin  => unimplemented!(),
                            CmpOp::Gte  => unimplemented!(),
                            CmpOp::Gt   => unimplemented!(),
                        } } } }
            Ok(acc)
        } } }


pub fn eval_sum<'a>(context: &'a Context, expr: &'a Expr) -> EvalResult<Cow<'a, BType>> {
    let mut itr = expr.sum.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ExprItem(_, ref term)) => {
            let mut acc = eval_prod(context, term)?;
            for &ExprItem(ref op, ref term) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_prod(context, term)? {
                        None => return Ok(None),
                        Some(term) => match *op {
                            SumOp::Add => acc.as_ref().try_add(term),
                            SumOp::Sub => acc.as_ref().try_sub(term),
                        } } } }
            Ok(acc)
        } } }


pub fn eval_prod<'a>(context: &'a Context, term: &'a Term) -> EvalResult<Cow<'a, BType>> {
    let mut itr = term.mul.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&TermItem(_, ref factor)) => {
            let mut acc = eval_factor(context, factor)?;
            for &TermItem(ref op, ref factor) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_factor(context, factor)? {
                        None => return Ok(None),
                        Some(factor) => match *op {
                            MulOp::Mul => acc.as_ref().try_mul(factor),
                            MulOp::Div => acc.as_ref().try_div(factor),
                        } } } }
            Ok(acc)
        } } }


pub fn eval_factor<'a>(context: &'a Context, fctr: &'a Factor) -> EvalResult<Cow<'a, BType>> {
    match *fctr {
        Factor::Variable(ref id)        => Ok(context.get(id).map(Cow::Borrowed)),
        Factor::Literal(ref lit)        => literal(lit).map(|v| v.map(Cow::Owned)),
        Factor::Subexpression(ref expr) => eval_expr(context, expr),
        Factor::Attribute(ref attr)     => eval_attribute(context, attr),
        Factor::Invocation(ref inv)     => eval_invocation(context, inv),
    }
}


pub fn eval_attribute<'a>(context: &'a Context, attr: &'a Attribute) -> EvalResult<Cow<'a, BType>> {
    match eval_factor(context, &attr.on)? {
        None => Err(EvalError::NotComposable),
        Some(value) => match value.try_as_composable() {
            None => Err(EvalError::NotComposable),
            Some(composable) => match composable.get_attr(&attr.id) {
                None => Err(EvalError::AttributeNotExists(attr.id.clone())),
                Some(result) => Ok(Some(Cow::Owned(result))),
            } } } }


pub fn eval_invocation<'a>(context: &'a Context, inv: &'a Invocation) -> EvalResult<Cow<'a, BType>> {
    match eval_factor(context, &inv.on)? {
        None => Err(EvalError::NotInvocable),
        Some(value) => match value.try_as_invocable() {
            None => Err(EvalError::NotInvocable),
            Some(invocable) => {
                let mut args: Vec<Cow<BType>> = Vec::with_capacity(inv.args.len());
                for expr in &inv.args {
                    match eval_expr(context, expr)? {
                        None => return Err(EvalError::NoneArg),
                        Some(val) => args.push(val)
                    }
                }
                invocable.invoke(args.as_slice(), context)
            } } } }


pub fn literal<'a>(l: &'a Literal) -> EvalResult<BType> {
    Ok( Some( match *l {
        Literal::Str(ref string) => BType(box string.clone()),
        Literal::Char(ref chr)   => BType(box *chr),
        Literal::Int(ref int)    => BType(box *int),
        Literal::Real(ref real)  => BType(box *real),
    } ) )
}


#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    use std::borrow::Cow;
    use nom::IResult;
    use std::fmt::Debug;

    fn unwrap_iresult<B: Debug, T>(result: IResult<B, T>) -> T {
        match result {
            IResult::Done(_, v) => v,
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        }
    }

    #[test]
    fn eval_attr() {
        use abc::EvalResult;
        use incrust::{Incrust, BType, Args, ex};
        use parser::expressions::expression as parse_expr;
        use super::eval_expr;

        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let context = incrust.context(&args);

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult<Cow<BType>>| {
            r.unwrap().unwrap().as_ref()
                .try_as_string()
                .map(|c| c.into_owned())
                .unwrap()
        };

        assert_eq!("1"   , x(eval_expr(&context, &parse(br#""a".length"#))));
        assert_eq!("2"   , x(eval_expr(&context, &parse(br#"("a" + "b").length"#))));
        assert_eq!("5"   , x(eval_expr(&context, &parse(br#"the_one . length"#))));
    }

    #[test]
    fn eval_factor() {
        use abc::EvalResult;
        use template::{Factor, Literal};
        use incrust::{Incrust, BType, Args, ex};
        use parser::expressions::expression as parse_expr;
        use super::eval_factor;
        use super::eval_expr;

        let int_one: Factor = Literal::Int(1_i64).into();
        let the_one = Factor::Variable("the_one".into());

        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let context = incrust.context(&args);

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult<Cow<BType>>| {
            r.unwrap().unwrap().as_ref()
                .try_as_string()
                .map(|c| c.into_owned())
                .unwrap()
        };

        assert!("1"      == x(eval_factor(&context, &int_one)));
        assert!("World"  == x(eval_factor(&context, &the_one)));
        assert!("Space"  != x(eval_factor(&context, &the_one)));

        assert_eq!("1"    , x(eval_expr(&context, &parse(b"1"))));
        assert_eq!("2"    , x(eval_expr(&context, &parse(b"1 + 1"))));
        assert_eq!("0"    , x(eval_expr(&context, &parse(b"1 - 1"))));
        assert_eq!("1"    , x(eval_expr(&context, &parse(b"2 / 2"))));
        assert_eq!("1"    , x(eval_expr(&context, &parse(b"3 / 2.0"))));
        assert_eq!("1.5"  , x(eval_expr(&context, &parse(b"3.0 / 2"))));

        assert_eq!("ab"   , x(eval_expr(&context, &parse(br#""a" + "b""#))));
        assert_eq!("ab"   , x(eval_expr(&context, &parse(br#"("a" + "b")"#))));
    }

    #[test]
    fn eval_bool() {
        use abc::EvalResult;
        use incrust::{Incrust, BType, Args, ex};
        use parser::expressions::expression as parse_expr;
        use super::eval_expr;

        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let context = incrust.context(&args);

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult<Cow<BType>>| {
            r.unwrap().unwrap().as_ref()
                .try_as_string()
                .map(|c| c.into_owned())
                .unwrap()
        };

        assert_eq!("1",     x(eval_expr(&context, &parse(b"0 or 1"))));
        assert_eq!("0",     x(eval_expr(&context, &parse(b"0 and 1"))));
        assert_eq!("str",   x(eval_expr(&context, &parse(br#""" or "str""#))));
        assert_eq!("",      x(eval_expr(&context, &parse(br#""" and "str""#))));
        assert_eq!("2",     x(eval_expr(&context, &parse(br#"0 and 1 or 2"#))));
        assert_eq!("2",     x(eval_expr(&context, &parse(br#"0 or 1 and 2"#))));
        assert_eq!("1",     x(eval_expr(&context, &parse(br#"0 or 1 or 2"#))));
        assert_eq!("1",     x(eval_expr(&context, &parse(br#"1 or 0 and 2"#))));
    }

//    #[test]
//    fn eval_expr() {
//        use ::abc::EvalResult;
//        use ::template::{Expr, ExprItem, Factor, Literal};
//        use ::incrust::{Incrust, Context, Args, ex, Type, BType};
//
//        let int_one: Factor = Literal::Int(1isize).into();
//
//        let args: Args = hashmap!{ "two" => ex(2isize) };
//        let context = Context::new(&Default::default(), &args);
//
//        let x = |r: EvalResult| r.unwrap().unwrap().to_istring().unwrap();
//
//        assert!("1"     == x(super::eval_factor(&int_one, &context)));
//        assert!("World" == x(super::eval_factor(&the_one, &context)));
//        assert!("Space" != x(super::eval_factor(&the_one, &context)));
//    }
}
