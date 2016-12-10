use std::fmt;

use abc::{RenderResult, FilterResult};
use incrust::{Context, Args, ex};
use template::{
    Parsed, Mustache,
    FullExpression, FilterItem,
    IfStatement, ForStatement,
};

use super::evaluator::eval_expr;


pub fn text<'a>(context: &'a Context, tpl: &'a[Parsed]) -> RenderResult<String> {
    let mut buffer: String = Default::default();
    render_text(&mut buffer, context, tpl)?;
    Ok(buffer)
}


pub fn render_text<'a, W: fmt::Write>(writer: &mut W, context: &'a Context, tpl: &'a[Parsed]) -> RenderResult<()> {
    for x in tpl {
        match *x {
            Parsed::Text(ref txt) => write!(writer, "{}", txt)?,
            Parsed::Comment(_) => (),
            Parsed::Mustache(ref mus) => render_mustache(writer, context, mus)?,
            Parsed::For(ref stmt) => render_for(writer, context, stmt)?,
            Parsed::If(ref stmt) => render_if(writer, context, stmt)?,
        }
    }
    Ok(())
}


pub fn render_mustache<W: fmt::Write>(writer: &mut W, context: &Context, mus: &Mustache) -> RenderResult<()> {
    render_expression(writer, context, &mus.expr)
}


pub fn render_expression<W: fmt::Write>(writer: &mut W, context: &Context, expr: &FullExpression) -> RenderResult<()> {
    if let Some(x) = expr.filters.iter().fold(
        Ok(eval_expr(context, &expr.expr)?.and_then(|val| val.try_as_string().map(|s| s.into_owned()))),
        |result: FilterResult, filter: &FilterItem| -> FilterResult {
            match result {
                Err(err)    => Err(err),
                Ok(value)   => Ok(match *filter {
                    FilterItem::Simple(ref id) => context.env().filter(id, value, context)?,
                }),
            }
        }
    )? {
        write!(writer, "{}", x)?;
    }
    Ok(())
}


pub fn render_for<W: fmt::Write>(writer: &mut W, context: &Context, stmt: &ForStatement) -> RenderResult<()> {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    // FIXME implement instead: expression(&stmt.begin.expression, context)
    if let Some(ref expr) = stmt.begin.expression {
        if let Some(value) = eval_expr(context, &expr.expr)? {
            if let Some(iterable) = value.try_as_iterable() {
                for (index, v) in iterable.ivalues().enumerate() {
                    let local_scope: Args = hashmap! {
                        stmt.value_var.as_str().into() => v,
                        "index0".into() => ex(index as i64),
                        "index".into() => ex(index as i64 + 1),
                        "first".into() => ex(index == 0),
                        "last".into() => ex(false), // TODO the "last" marker in a loop
                    };
                    render_text(writer, &context.nest(&local_scope), &stmt.block.parsed)?;
                }
            }
        }
    };
    Ok(())
}


pub fn render_if<W: fmt::Write>(writer: &mut W, context: &Context, stmt: &IfStatement) -> RenderResult<()> {
    for branch in &stmt.if_branches {
        // FIXME implement instead: expression(&branch.begin.expression, context)
        if let Some(ref expr) = branch.begin.expression {
            if let Some(res) = eval_expr(context, &expr.expr)? {
                if res.to_bool() {
                    render_text(writer, context, &branch.block.parsed)?;
                    return Ok(());
                }
            }
        }
    }
    if let Some(ref branch) = stmt.else_branch {
        render_text(writer, context, &branch.block.parsed)?;
    }
    Ok(())
}
