use ::abc::EvalResult;
use ::incrust::{Incrust, Context};
use ::template::{
    Expr, Term,
    SumOp, MulOp,
    ExprItem, TermItem,
    Factor, Literal,
};

pub fn eval_expr<'a>(expr: &'a Expr, context: &'a Context, env: &'a Incrust) -> EvalResult {
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
                            SumOp::Or => unimplemented!(),
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
                            MulOp::And => unimplemented!(),
                        } } } }
            Ok(acc)
        } } }

pub fn eval_factor<'a>(fctr: &'a Factor, context: &'a Context, env: &'a Incrust) -> EvalResult {
    Ok(match *fctr {
        Factor::Literal(ref lit) => literal(lit, context, env)?,
        Factor::Subexpression(ref expr) => eval_expr(expr, context, env)?,
        Factor::Variable(ref id) => match context.get(id) {
            Some(v) => Some(v.iclone()?),
            None => None,
        },
    })
}

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
//    use ::parser::expressions::;
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
    fn eval_factor() {
        use ::abc::EvalResult;
        use ::template::{Factor, Literal};
        use ::incrust::{Incrust, Context, Args, ex};
        use ::parser::expressions::sum as parse_expr;

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
