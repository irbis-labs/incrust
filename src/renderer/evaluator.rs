use abc::{EvalResult, EvalError, InvokeError};
use container::expression::*;
use {Arg, Context};


pub fn eval_expr<'r>(context: &'r Context<'r>, disj_expr: &'r DisjExpr) -> EvalResult<Arg<'r>> {
    let mut itr = disj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(conj) => {
            let mut acc = eval_conj(context, conj)?;
            for conj in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(a) => match a.to_bool() {
                        true => return Ok(Some(a)),
                        false => eval_conj(context, conj)?,
                    } } }
            Ok(acc)
        } } }


pub fn eval_conj<'r>(context: &'r Context<'r>, conj_expr: &'r ConjExpr) -> EvalResult<Arg<'r>> {
    let mut itr = conj_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(cmp) => {
            let mut acc = eval_cmp(context, cmp)?;
            for cmp in itr {
                acc = match acc {
                    // FIXME eval None as False?
                    None => return Ok(None),
                    Some(acc) => match acc.to_bool() {
                        true => eval_cmp(context, cmp)?,
                        false => return Ok(Some(acc))
                    } } }
            Ok(acc)
        } } }


#[allow(unused_variables)]
pub fn eval_cmp<'r>(context: &'r Context<'r>, cmp_expr: &'r CmpExpr) -> EvalResult<Arg<'r>> {
    let mut itr = cmp_expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&CmpItem(_, ref expr)) => {
            let mut acc = eval_sum(context, expr)?;
            for &CmpItem(ref op, ref expr) in itr {
                acc = match acc {
                    None => return Ok(None),
                    Some(acc) => match eval_sum(context, expr)? {
                        None => return Ok(None),
                        Some(expr) => match *op {
                            // todo error if is not partial_eq ?
                            CmpOp::Eq   => acc.try_as_partial_eq().map(|acc| acc.eq(&expr)),
                            CmpOp::Neq  => acc.try_as_partial_eq().map(|acc| acc.ne(&expr)),
                            CmpOp::Lt   => acc.try_as_partial_ord().and_then(|acc| acc.lt(&expr)),
                            CmpOp::Gt   => acc.try_as_partial_ord().and_then(|acc| acc.gt(&expr)),
                            CmpOp::Lte  => acc.try_as_partial_ord().and_then(|acc| acc.le(&expr)),
                            CmpOp::Gte  => acc.try_as_partial_ord().and_then(|acc| acc.ge(&expr)),
                            CmpOp::In   => unimplemented!(),
                            CmpOp::Nin  => unimplemented!(),
                        }
                            .map(Arg::from)
                    } } }
            Ok(acc)
        } } }


pub fn eval_sum<'r>(context: &'r Context<'r>, expr: &'r Expr) -> EvalResult<Arg<'r>> {
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
                            SumOp::Add => acc.try_add(term),
                            SumOp::Sub => acc.try_sub(term),
                        } } } }
            Ok(acc)
        } } }


pub fn eval_prod<'r>(context: &'r Context<'r>, term: &'r Term) -> EvalResult<Arg<'r>> {
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
                            MulOp::Mul => acc.try_mul(factor),
                            MulOp::Div => acc.try_div(factor),
                        } } } }
            Ok(acc)
        } } }


pub fn eval_factor<'r>(context: &'r Context<'r>, fctr: &'r Factor) -> EvalResult<Arg<'r>> {
    match *fctr {
        Factor::Variable(ref id)        => Ok(context.get(id)),
        Factor::Literal(ref lit)        => literal(lit),
        Factor::Subexpression(ref expr) => eval_expr(context, expr),
        Factor::Index(ref index)        => eval_index(context, index),
        Factor::Attribute(ref attr)     => eval_attribute(context, attr),
        Factor::Invocation(ref inv)     => eval_invocation(context, inv),
    }
}


pub fn eval_index<'r>(context: &'r Context<'r>, index: &'r Index) -> EvalResult<Arg<'r>> {
    match eval_factor(context, &index.on)? {
        None => Err(EvalError::NotComposable),
        Some(value) => {
            if let Some(key) = eval_expr(context, &index.index)? {
                if let Some(key) = key.try_as_int() {
                    return match value.try_as_indexable() {
                        None => Err(EvalError::NotIndexable),
                        // TODO consider negative values
                        Some(indexable) => match indexable.get_index(key as usize) {
                            None => Err(EvalError::IndexNotExists(key as usize)),
                            // fixme extra clone
                            Some(result) => Ok(Some(result.to_owned())),
                        } } }

                if let Some(key) = key.try_as_string() {
                    return match value.try_as_mappable() {
                        None => Err(EvalError::NotMappable),
                        Some(mappable) => match mappable.get_by_key(key.as_ref()) {
                            None => Err(EvalError::KeyNotExists(key.into_owned())),
                            // fixme extra clone
                            Some(result) => Ok(Some(result.to_owned())),
                        } } }
            }
            Err(EvalError::UnexpectedIndexType)
        } } }


pub fn eval_attribute<'r>(context: &'r Context<'r>, attr: &'r Attribute) -> EvalResult<Arg<'r>> {
    match eval_factor(context, &attr.on)? {
        None => Err(EvalError::NotComposable),
        Some(value) => {
            match value.try_as_composable() {
                None => Err(EvalError::NotComposable),
                Some(composable) => match composable.get_attr(&attr.id) {
                    None => Err(EvalError::AttributeNotExists(attr.id.clone())),
                    // fixme extra clone
                    Some(result) => Ok(Some(result.to_owned())),
                } } } } }


pub fn eval_invocation<'r>(context: &'r Context<'r>, inv: &'r Invocation) -> EvalResult<Arg<'r>> {
    match eval_factor(context, &inv.on)? {
        None => Err(InvokeError::NotInvocable)?,
        Some(value) => match value.try_as_invocable() {
            None => Err(InvokeError::NotInvocable)?,
            Some(invocable) => {
                let mut args: Vec<Arg> = Vec::with_capacity(inv.args.len());
                for expr in &inv.args {
                    match eval_expr(context, expr)? {
                        None => Err(EvalError::NoneArg)?,
                        Some(val) => args.push(val)
                    }
                }
                invocable.invoke(args.as_slice(), context)
            } } } }


pub fn literal<'r>(l: &'r Literal) -> EvalResult<Arg<'r>> {
    Ok( Some( match *l {
        Literal::Str(ref string) => Arg::from(string.clone()),
        Literal::Char(ref chr)   => Arg::from(*chr),
        Literal::Int(ref int)    => Arg::from(*int),
        Literal::Real(ref real)  => Arg::from(*real),
    } ) )
}


#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    use nom::IResult;
    use std::fmt::Debug;

    use abc::*;
    use {Incrust, Arg, Args, Context, ex, Template};
    use parser::expressions::expression as parse_expr;
    use super::eval_expr;


    fn unwrap_iresult<B: Debug, T>(result: IResult<B, T>) -> T {
        match result {
            IResult::Done(_, v) => v,
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        }
    }

    fn test_eval_expr(context: &Context, a: &str, b: &[u8]) {
        let b = unwrap_iresult(parse_expr(b));
        let r = eval_expr(&context, &b);
        let res = r.unwrap().unwrap()
            .try_as_string()
            .map(|c| c.into_owned())
            .unwrap();
        assert_eq!(a, res)
    }


    #[test]
    fn eval_attr() {
        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let template = Template::default();
        let context = incrust.create_global_context(&template, &args).unwrap();
        let context = context.top_scope();

        test_eval_expr(&context, "1", br#""a".length"#);
        test_eval_expr(&context, "2", br#"("a" + "b").length"#);
        test_eval_expr(&context, "5", br#"the_one . length"#);
    }

    #[test]
    fn eval_factor() {
        use container::expression::{Factor, Literal};
        use super::eval_factor;

        let int_one: Factor = Literal::Int(1_i64).into();
        let the_one = Factor::Variable("the_one".into());

        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let template = Template::default();
        let context = incrust.create_global_context(&template, &args).unwrap();
        let context = context.top_scope();

        let x = |r: EvalResult<Arg>| {
            r.unwrap().unwrap()
                .try_as_string()
                .map(|c| c.into_owned())
                .unwrap()
        };

        assert!("1"      == x(eval_factor(&context, &int_one)));
        assert!("World"  == x(eval_factor(&context, &the_one)));
        assert!("Space"  != x(eval_factor(&context, &the_one)));

        test_eval_expr(&context, "1",   b"1");
        test_eval_expr(&context, "2",   b"1 + 1");
        test_eval_expr(&context, "0",   b"1 - 1");
        test_eval_expr(&context, "1",   b"2 / 2");
        test_eval_expr(&context, "1",   b"3 / 2.0");
        test_eval_expr(&context, "1.5", b"3.0 / 2");

        test_eval_expr(&context, "ab",  br#""a" + "b""#);
        test_eval_expr(&context, "ab",  br#"("a" + "b")"#);
    }

    #[test]
    fn eval_bool() {
        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let template = Template::default();
        let context = incrust.create_global_context(&template, &args).unwrap();
        let context = context.top_scope();

        test_eval_expr(&context, "1",     b"0 or 1");
        test_eval_expr(&context, "0",     b"0 and 1");
        test_eval_expr(&context, "str",   br#""" or "str""#);
        test_eval_expr(&context, "",      br#""" and "str""#);
        test_eval_expr(&context, "2",     br#"0 and 1 or 2"#);
        test_eval_expr(&context, "2",     br#"0 or 1 and 2"#);
        test_eval_expr(&context, "1",     br#"0 or 1 or 2"#);
        test_eval_expr(&context, "1",     br#"1 or 0 and 2"#);
    }

    #[test]
    fn compare() {
        let args: Args = hashmap!{ "the_one".into() => ex("World") };
        let incrust = Incrust::default();
        let template = Template::default();
        let context = incrust.create_global_context(&template, &args).unwrap();
        let context = context.top_scope();

        test_eval_expr(&context, "true",  b"1 == 1");
        test_eval_expr(&context, "true",  b"1 != 2");
        test_eval_expr(&context, "false", b"1 == 2");

        test_eval_expr(&context, "true",  br#""1" == "1""#);
        test_eval_expr(&context, "true",  br#""1" != "2""#);
        test_eval_expr(&context, "false", br#""1" == "2""#);

        test_eval_expr(&context, "false", br#""1" == 1"#);
        test_eval_expr(&context, "true",  br#""1" != 1"#);

        test_eval_expr(&context, "false", br#"1 == "1""#);
        test_eval_expr(&context, "true",  br#"1 != "1""#);

        test_eval_expr(&context, "true",  b"1 <= 1");
        test_eval_expr(&context, "false", b"1 < 1");
        test_eval_expr(&context, "true",  b"1 <= 2");
        test_eval_expr(&context, "true",  b"1 < 2");
        test_eval_expr(&context, "false", b"1 >= 2");
        test_eval_expr(&context, "false", b"1 > 2");

        test_eval_expr(&context, "true",  b"1 >= 1");
        test_eval_expr(&context, "false", b"1 > 1");
        test_eval_expr(&context, "true",  b"1 >= 1");
        test_eval_expr(&context, "true",  b"2 > 1");
        test_eval_expr(&context, "false", b"2 <= 1");
        test_eval_expr(&context, "false", b"2 < 1");
    }

//    #[test]
//    fn eval_expr() {
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
