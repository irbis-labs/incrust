use ::abc::{EvalResult, EvalError};
use ::incrust::{Incrust, Context};
use ::template::{
    DisjExpr, ConjExpr, CmpExpr, Expr, Term,
    DisjOp, ConjOp, CmpOp, SumOp, MulOp,
    DisjItem, ConjItem, CmpItem, ExprItem, TermItem,
    Factor, Literal,
};

pub fn eval_expr<'a>(disj_expr: &'a DisjExpr, context: &'a Context, env: &'a Incrust) -> EvalResult {
    let mut itr = disj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&DisjItem(ref _op, ref conj)) => {
            let mut acc = eval_conj(conj, context, env)?;
            for &DisjItem(ref op, ref conj) in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(acc) => match *op {
                        DisjOp::Or => match acc.to_bool() {
                            true => return Ok(Some(acc)),
                            false => eval_conj(conj, context, env)?,
                        },
                    } } }
            Ok(acc)
        } } }

pub fn eval_conj<'a>(conj_expr: &'a ConjExpr, context: &'a Context, env: &'a Incrust) -> EvalResult {
    let mut itr = conj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ConjItem(ref _op, ref cmp)) => {
            let mut acc = eval_cmp(cmp, context, env)?;
            for &ConjItem(ref op, ref cmp) in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(acc) => match *op {
                        ConjOp::And => match acc.to_bool() {
                            true => eval_cmp(cmp, context, env)?,
                            false => return Ok(Some(acc))
                        },
                    } } }
            Ok(acc)
        } } }

#[allow(unused_variables)]
pub fn eval_cmp<'a>(cmp_expr: &'a CmpExpr, context: &'a Context, env: &'a Incrust) -> EvalResult {
    let mut itr = cmp_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&CmpItem(ref _op, ref expr)) => {
            let acc = eval_sum(expr, context, env)?;
            for &CmpItem(ref op, ref expr) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_sum(expr, context, env)? {
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

pub fn eval_sum<'a>(expr: &'a Expr, context: &'a Context, env: &'a Incrust) -> EvalResult {
    let mut itr = expr.sum.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ExprItem(ref _op, ref term)) => {
            let start = eval_prod(term, context, env);
            itr.fold(start, |acc: EvalResult, &ExprItem(ref op, ref term)| -> EvalResult {
                Ok(match acc? {
                    None => None,
                    Some(acc) => match eval_prod(term, context, env)? {
                        None => None,
                        Some(term) => match *op {
                            SumOp::Add => acc.iadd(term),
                            SumOp::Sub => acc.isub(term),
                        } } } )
            } ) } } }

pub fn eval_prod<'a>(term: &'a Term, context: &'a Context, env: &'a Incrust) -> EvalResult {
    let mut itr = term.mul.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&TermItem(ref _op, ref factor)) => {
            let mut acc = eval_factor(factor, context, env)?;
            for &TermItem(ref op, ref factor) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_factor(factor, context, env)? {
                        None => return Ok(None),
                        Some(factor) => match *op {
                            MulOp::Mul => acc.imul(factor),
                            MulOp::Div => acc.idiv(factor),
                        } } } }
            Ok(acc)
        } } }

pub fn eval_factor<'a>(fctr: &'a Factor, context: &'a Context, env: &'a Incrust) -> EvalResult {
    Ok(match *fctr {
        Factor::Literal(ref lit) => literal(lit, context, env)?,
        Factor::Subexpression(ref expr) => eval_expr(expr, context, env)?,
        Factor::Variable(ref id) => match context.get(id) {
            None => None,
            Some(v) => Some(v.iclone().map_err(|err| EvalError::Input(format!("{:?}", err)))?),
        },
        Factor::Attribute(ref attr) => {
            match eval_factor(&attr.on, context, env)? {
                None => Err(EvalError::NotComposable)?,
                Some(value) => {
                    match value.as_composable() {
                        None => Err(EvalError::NotComposable)?,
                        Some(composable) => match composable.get_attr(&attr.id) {
                            None => Err(EvalError::AttributeNotExists(attr.id.clone()))?,
                            // FIXME why need to clone?
                            Some(result) => Some(result.iclone().map_err(|err| EvalError::Input(format!("{:?}", err)))?),
                        }
                    }
                }
            }
        }
    })
}

//use ::types::abc::{IClone, CloneError, BType};
//
//pub fn clone_value(v: &IClone) -> Result<BType, CloneError> {
//
//}

pub fn literal<'a>(l: &'a Literal, _context: &'a Context, _env: &'a Incrust) -> EvalResult {
    Ok(Some(match *l {
        Literal::Str(ref string) => Box::new(string.clone()),
        Literal::Char(ref chr) => Box::new(*chr),
        Literal::Int(ref int) => Box::new(*int),
        Literal::Real(ref real) => Box::new(*real),
    }))
}




#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
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
        use ::abc::EvalResult;
        use ::incrust::{Incrust, Context, Args, ex};
        use ::parser::expressions::expression as parse_expr;

        let args: Args = hashmap!{ "the_one" => ex("World") };
        let context = Context::new(None, &args);
        let incrust = Incrust::new();

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult| r.unwrap().unwrap().to_istring().unwrap();

        assert_eq!("1"   , x(super::eval_expr(&parse(br#""a".length"#), &context, &incrust)));
        assert_eq!("2"   , x(super::eval_expr(&parse(br#"("a" + "b").length"#), &context, &incrust)));
        assert_eq!("5"   , x(super::eval_expr(&parse(br#"the_one . length"#), &context, &incrust)));
    }

    #[test]
    fn eval_factor() {
        use ::abc::EvalResult;
        use ::template::{Factor, Literal};
        use ::incrust::{Incrust, Context, Args, ex};
        use ::parser::expressions::expression as parse_expr;

        let int_one: Factor = Literal::Int(1isize).into();
        let the_one = Factor::Variable("the_one".into());

        let args: Args = hashmap!{ "the_one" => ex("World") };
        let context = Context::new(None, &args);
        let incrust = Incrust::new();

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult| r.unwrap().unwrap().to_istring().unwrap();

        assert!("1"      == x(super::eval_factor(&int_one, &context, &incrust)));
        assert!("World"  == x(super::eval_factor(&the_one, &context, &incrust)));
        assert!("Space"  != x(super::eval_factor(&the_one, &context, &incrust)));

        assert_eq!("1"    , x(super::eval_expr(&parse(b"1"), &context, &incrust)));
        assert_eq!("2"    , x(super::eval_expr(&parse(b"1 + 1"), &context, &incrust)));
        assert_eq!("0"    , x(super::eval_expr(&parse(b"1 - 1"), &context, &incrust)));
        assert_eq!("1"    , x(super::eval_expr(&parse(b"2 / 2"), &context, &incrust)));
        assert_eq!("1"    , x(super::eval_expr(&parse(b"3 / 2.0"), &context, &incrust)));
        assert_eq!("1.5"  , x(super::eval_expr(&parse(b"3.0 / 2"), &context, &incrust)));

        assert_eq!("ab"   , x(super::eval_expr(&parse(br#""a" + "b""#), &context, &incrust)));
        assert_eq!("ab"   , x(super::eval_expr(&parse(br#"("a" + "b")"#), &context, &incrust)));
    }

    #[test]
    fn eval_bool() {
        use ::abc::EvalResult;
        use ::incrust::{Incrust, Context, Args, ex};
        use ::parser::expressions::expression as parse_expr;

        let args: Args = hashmap!{ "the_one" => ex("World") };
        let context = Context::new(None, &args);
        let incrust = Incrust::new();

        let parse = |s| unwrap_iresult(parse_expr(s));
        let x = |r: EvalResult| r.unwrap().unwrap().to_istring().unwrap();

        assert_eq!("1"      , x(super::eval_expr(&parse(b"0 or 1"), &context, &incrust)));
        assert_eq!("0"      , x(super::eval_expr(&parse(b"0 and 1"), &context, &incrust)));
        assert_eq!("str"    , x(super::eval_expr(&parse(br#""" or "str""#), &context, &incrust)));
        assert_eq!(""       , x(super::eval_expr(&parse(br#""" and "str""#), &context, &incrust)));
        assert_eq!("2"      , x(super::eval_expr(&parse(br#"0 and 1 or 2"#), &context, &incrust)));
        assert_eq!("2"      , x(super::eval_expr(&parse(br#"0 or 1 and 2"#), &context, &incrust)));
        assert_eq!("1"      , x(super::eval_expr(&parse(br#"0 or 1 or 2"#), &context, &incrust)));
        assert_eq!("1"      , x(super::eval_expr(&parse(br#"1 or 0 and 2"#), &context, &incrust)));
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
//        let context = Context::new(None, &args);
//        let incrust = Incrust::new();
//
//        let x = |r: EvalResult| r.unwrap().unwrap().to_istring().unwrap();
//
//        assert!("1"     == x(super::eval_factor(&int_one, &context, &incrust)));
//        assert!("World" == x(super::eval_factor(&the_one, &context, &incrust)));
//        assert!("Space" != x(super::eval_factor(&the_one, &context, &incrust)));
//    }
}
