use ::abc::{RenderResult, FilterResult};
use ::incrust::{Incrust, Context};
use ::template::{
    Parsed, Mustache, ForEach,
    FullExpression, FilterItem,
    IfStatement,
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
            Parsed::ForEach(ref fe) => foreach(fe, context, env)?,
            Parsed::If(ref stmt) => if_statement(stmt, context, env)?,
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


#[allow(unused_variables, dead_code)]
pub fn foreach(fe: &ForEach, context: &Context, env: &Incrust) -> RenderResult {
    Ok("".into())
}


pub fn if_statement(stmt: &IfStatement, context: &Context, env: &Incrust) -> RenderResult {
    for branch in &stmt.if_branches {
        // FIXME use instead: expression(&branch.begin.expression, context, env)
        if let Some(ref expr) = branch.begin.expression {
            if let Some(res) = eval_expr(&expr.expr, context, env)? {
                if res.to_bool() { return text(&branch.block.parsed, context, env) }
            }
        }
    }
    if let Some(ref branch) = stmt.else_branch{ return text(&branch.block.parsed, context, env) }
    Ok("".into())
}
