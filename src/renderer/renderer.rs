use ::abc::{RenderResult, FilterResult};
use ::incrust::{Incrust, Context, Args, ex};
use ::template::{
    Parsed, Mustache,
    FullExpression, FilterItem,
    IfStatement, ForStatement,
};

use super::evaluator::eval_expr;


//pub fn text<'a>(buffer: &mut[u8], tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> RenderResult {
pub fn text<'a>(tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> RenderResult {
    let mut res: Vec<String> = Vec::new();
    for x in tpl {
        res.push(match *x {
            Parsed::Text(ref txt) => txt.to_owned(),
            Parsed::Comment(_) => "".to_owned(),
            Parsed::Mustache(ref mus) => mustache(mus, context, env)?,
            Parsed::For(ref stmt) => for_(stmt, context, env)?,
            Parsed::If(ref stmt) => if_(stmt, context, env)?,
        })
    }
    Ok(res.join(""))
}

//pub fn mustache(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> RenderResult {
pub fn mustache(mus: &Mustache, context: &Context, env: &Incrust) -> RenderResult {
    expression(&mus.expr, context, env)
}


//pub fn full_expression(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> RenderResult {
pub fn expression(expr: &FullExpression, context: &Context, env: &Incrust) -> RenderResult {
    Ok(expr.filters.iter().fold(
        Ok(eval_expr(&expr.expr, context, env)?.and_then(|val| val.to_istring())),
        |result: FilterResult, filter: &FilterItem| -> FilterResult {
            match result {
                Err(err)    => Err(err),
                Ok(value)   => Ok(match *filter {
                    FilterItem::Simple(ref id) => env.filter(id, value, context)?,
                }),
            }
        }
    )?.unwrap_or("".into()))
}


pub fn for_(stmt: &ForStatement, context: &Context, env: &Incrust) -> RenderResult {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]

    // FIXME implement instead: expression(&stmt.begin.expression, context, env)
    Ok(match stmt.begin.expression {
        None => "".into(),
        Some(ref expr) => {
            let value = eval_expr(&expr.expr, context, env)?;
            match value {
                None => "".into(),
                Some(value) => match value.as_iterable() {
                    None => "".into(),
                    Some(iterable) => {
                        let mut buf: Vec<String> = Vec::new();
                        for (index, v) in iterable.ivalues().enumerate() {
                            let local_scope: Args = hashmap!{
                                stmt.value_var.as_str() => v,
                                "index0" => ex(index as isize),
                                "index" => ex(index as isize + 1),
                                "first" => ex(index == 0),
                                "last" => ex(false), // TODO the "last" marker in a loop
                            };
                            let local_context = Context::new(Some(context), &local_scope);
                            buf.push(text(&stmt.block.parsed, &local_context, env)?);
                        }
                        buf.join("")
                    }
                }
            }
        }
    })
}


pub fn if_(stmt: &IfStatement, context: &Context, env: &Incrust) -> RenderResult {
    for branch in &stmt.if_branches {
        // FIXME implement instead: expression(&branch.begin.expression, context, env)
        if let Some(ref expr) = branch.begin.expression {
            if let Some(res) = eval_expr(&expr.expr, context, env)? {
                if res.to_bool() { return text(&branch.block.parsed, context, env) }
            }
        }
    }
    if let Some(ref branch) = stmt.else_branch{ return text(&branch.block.parsed, context, env) }
    Ok("".into())
}
