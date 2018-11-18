use std::fmt;

use abc::RenderResult;
use VarContext;
use container::expression::*;


pub fn render_expr<W: fmt::Write>(writer: &mut W, context: &VarContext, expr: &DisjExpr) -> RenderResult<()> {
    let mut itr = expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(conj) => {
            render_conj(writer, context, conj)?;
            for conj in itr {
                write!(writer, " or ")?;
                render_conj(writer, context, conj)?;
            }
            Ok(())
        } } }


pub fn render_conj<W: fmt::Write>(writer: &mut W, context: &VarContext, expr: &ConjExpr) -> RenderResult<()> {
    let mut itr = expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(cmp) => {
            render_cmp(writer, context, cmp)?;
            for cmp in itr {
                write!(writer, " and ")?;
                render_cmp(writer, context, cmp)?;
            }
            Ok(())
        } } }


pub fn render_cmp<W: fmt::Write>(writer: &mut W, context: &VarContext, expr: &CmpExpr) -> RenderResult<()> {
    let mut itr = expr.list.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&CmpItem(ref _op, ref sum)) => {
            render_sum(writer, context, sum)?;
            for &CmpItem(ref op, ref sum) in itr {
                let op = match *op {
                    CmpOp::Lt   => "<",
                    CmpOp::Lte  => "<=",
                    CmpOp::Eq   => "==",
                    CmpOp::Neq  => "!=",
                    CmpOp::In   => "in",
                    CmpOp::Nin  => "not in",
                    CmpOp::Gte  => ">=",
                    CmpOp::Gt   => ">",
                };
                write!(writer, " {} ", op)?;
                render_sum(writer, context, sum)?;
            }
            Ok(())
        } } }


pub fn render_sum<W: fmt::Write>(writer: &mut W, context: &VarContext, expr: &Expr) -> RenderResult<()> {
    let mut itr = expr.sum.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&ExprItem(ref _op, ref term)) => {
            render_prod(writer, context, term)?;
            for &ExprItem(ref op, ref term) in itr {
                let op = match *op {
                    SumOp::Add => "+",
                    SumOp::Sub => "-",
                };
                write!(writer, " {} ", op)?;
                render_prod(writer, context, term)?;
            }
            Ok(())
        } } }


pub fn render_prod<W: fmt::Write>(writer: &mut W, context: &VarContext, term: &Term) -> RenderResult<()> {
    let mut itr = term.mul.iter();
    match itr.next() {
        None => unreachable!(),
        Some(&TermItem(ref _op, ref factor)) => {
            render_factor(writer, context, factor)?;
            for &TermItem(ref op, ref factor) in itr {
                let op = match *op {
                    MulOp::Mul => "*",
                    MulOp::Div => "/",
                };
                write!(writer, " {} ", op)?;
                render_factor(writer, context, factor)?;
            }
            Ok(())
        } } }


pub fn render_factor<W: fmt::Write>(writer: &mut W, context: &VarContext, fctr: &Factor) -> RenderResult<()> {
    match *fctr {
        Factor::Literal(ref lit) => {
            render_literal(writer, lit)?;
        },
        Factor::Variable(ref id) => {
            write!(writer, "{}", id)?;
        },
        Factor::Subexpression(ref expr) => {
            write!(writer, "(")?;
            render_expr(writer, context, expr)?;
            write!(writer, ")")?;
        },
        Factor::Attribute(ref attr) => {
            render_factor(writer, context, &*attr.on)?;
            write!(writer, ".{}", attr.id)?;
        },
        Factor::Index(ref index) => {
            render_factor(writer, context, &*index.on)?;
            write!(writer, "[")?;
            render_expr(writer, context, &index.index)?;
            write!(writer, "]")?;
        },
        Factor::Invocation(ref inv) => {
            render_factor(writer, context, &*inv.on)?;
            write!(writer, "(")?;
            for (i, expr) in inv.args.iter().enumerate() {
                if i != 0 { write!(writer, ", ")?; }
                render_expr(writer, context, expr)?;
            }
            write!(writer, ")")?;
        },
    };
    Ok(())
}


pub fn render_literal<W: fmt::Write>(writer: &mut W, l: &Literal) -> RenderResult<()> {
    match *l {
        Literal::Str(ref string) => write!(writer, "{:?}", string)?,
        Literal::Char(ref chr)   => write!(writer, "{:?}", chr)?,
        Literal::Int(ref int)    => write!(writer, "{}", int)?,
        Literal::Real(ref real)  => write!(writer, "{}", real)?,
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    #![allow(clippy::used_underscore_binding)]
    use std::fmt::Debug;

    use nom::IResult;

    use {Incrust, Args, ex, Template};
    use parser::expressions::expression as parse_expr;

    fn unwrap_iresult<B: Debug, T>(result: IResult<B, T>) -> T {
        match result {
            IResult::Done(_, v) => v,
            IResult::Error(e) => panic!("{:?}", e),
            IResult::Incomplete(i) => panic!("{:?}", i),
        }
    }

    #[test]
    fn eval_expr() {
        use super::render_expr;

        let args: Args = hashmap!{
            "the_one".into() => ex("World"),
            "one".into() => ex(1_i64),
            "two".into() => ex(2_i64),
        };
        let incrust = Incrust::default();
        let template = Template::default();
        let context = incrust.create_global_context(&template, &args).unwrap();
        let context = context.top_scope();

        let parse = |s| unwrap_iresult(parse_expr(s));
        let test = |s, b| {
            let mut buf = String::new();
            render_expr(&mut buf, &context, &parse(b)).unwrap();
            assert_eq!(s, buf)
        };

        test("1",                   b"1");
        test("1 + 1",               b"1+1");
        test("1 + 1",               b"1 + 1");
        test("1 - 1",               b"1 \n -\t1");

        test("(1 / 1)",             b"(1 / 1)");
        test("1 * 1",               b"1 * 1");
        test("1 + 1 * 1",           b"1 + 1 * 1");
        test("(1 + 1) * 1",         b"(1 + 1) * 1");
        test("(1 + (1 / 1)) * 1",   b"(1+(1/1))*1");

        test("True and False",      b"True and False");
        test("0 or 1 and 2",        b"0 or 1 and 2");

        test("1 == 1",              b"1 == 1");
        test("1 != 1",              b"1 != 1");
        test("1 <= 1",              b"1 <= 1");
        test("1 >= 1",              b"1 >= 1");
        test("1 < 1",               b"1 < 1");
        test("1 > 1",               b"1 > 1");
    }
}
