use ::abc::RenderResult;
use ::incrust::{Incrust, Context};
use ::template::{
    Expr, Term,
    SumOp, MulOp,
    ExprItem, TermItem,
    Factor, Literal,
};

pub fn render_expr<'a, 'b>(expr: &'a Expr, context: &'b Context, env: &'b Incrust) -> RenderResult {
    let mut itr = expr.sum.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ExprItem(ref _op, ref term)) => {
            let start = render_prod(term, context, env);
            itr.fold(start, |acc: RenderResult, &ExprItem(ref op, ref term)| -> RenderResult {
                Ok( format!("{} {} {}", acc?, match *op {
                    SumOp::Add => "+",
                    SumOp::Sub => "-",
                    SumOp::Or => "or",
                }, render_prod(term, context, env)? ) )
            } ) } } }


pub fn render_prod<'a, 'b>(term: &'a Term, context: &'b Context, env: &'b Incrust) -> RenderResult {
    let mut itr = term.mul.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&TermItem(ref _op, ref factor)) => {
            let mut acc = render_factor(factor, context, env)?;
            for &TermItem(ref op, ref factor) in itr {
                acc = format!("{} {} {}", acc, match *op {
                    MulOp::Mul => "*",
                    MulOp::Div => "/",
                    MulOp::And => "and",
                }, render_factor(factor, context, env)? )
            }
            Ok(acc)
        } } }


pub fn render_factor<'a, 'b>(fctr: &'a Factor, context: &'b Context, env: &'b Incrust) -> RenderResult {
    Ok(match *fctr {
        Factor::Literal(ref lit) => render_literal(lit, context, env)?,
        Factor::Subexpression(ref expr) => format!("({})", render_expr(expr, context, env)? ),
        Factor::Variable(ref id) => id.clone(),
    })
}

pub fn render_literal<'a, 'b>(l: &'a Literal, _context: &'b Context, _env: &'b Incrust) -> RenderResult {
    Ok(match *l {
        Literal::Str(ref string) => format!("{:?}", string),
        Literal::Char(ref chr) => format!("{:?}", chr),
        Literal::Int(ref int) => format!("{}", int),
        Literal::Real(ref real) => format!("{}", real),
    })
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
    fn eval_expr() {
        use ::incrust::{Incrust, Context, Args, ex};
        use ::parser::expressions::sum as parse_expr;

        let args: Args = hashmap!{
            "the_one" => ex("World"),
            "one" => ex(1isize),
            "two" => ex(2isize),
        };
        let args: Args = hashmap!{  };
        let context = Context::new(None, &args);
        let incrust = Incrust::new();

        let parse = |s| unwrap_iresult(parse_expr(s));

        assert_eq!("1"                , super::render_expr(&parse(b"1"), &context, &incrust).unwrap());
        assert_eq!("1 + 1"            , super::render_expr(&parse(b"1+1"), &context, &incrust).unwrap());
        assert_eq!("1 + 1"            , super::render_expr(&parse(b"1 + 1"), &context, &incrust).unwrap());
        assert_eq!("1 - 1"            , super::render_expr(&parse(b"1 \n -\t1"), &context, &incrust).unwrap());
        assert_eq!("(1 / 1)"          , super::render_expr(&parse(b"(1 / 1)"), &context, &incrust).unwrap());

        assert_eq!("1 * 1"            , super::render_expr(&parse(b"1 * 1"), &context, &incrust).unwrap());
        assert_eq!("1 + 1 * 1"        , super::render_expr(&parse(b"1 + 1 * 1"), &context, &incrust).unwrap());
        assert_eq!("(1 + 1) * 1"      , super::render_expr(&parse(b"(1 + 1) * 1"), &context, &incrust).unwrap());
        assert_eq!("(1 + (1 / 1)) * 1", super::render_expr(&parse(b"(1+(1/1))*1"), &context, &incrust).unwrap());
//        assert!("World" == x(super::eval_factor(&the_one, &context, &incrust)));
//        assert!("Space" != x(super::eval_factor(&the_one, &context, &incrust)));
    }
}
