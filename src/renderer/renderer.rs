use ::abc;
use ::incrust::{Incrust, Context};
use ::template::{
    Parsed, Mustache, ForEach,
    FullExpression, FilterItem,
};

use super::evaluator::eval_expr;


//pub fn text<'a>(buffer: &mut[u8], tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> abc::RenderResult {
pub fn text<'a>(tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> abc::RenderResult {
    let mut res: Vec<String> = Vec::new();
    for x in tpl {
        res.push(match *x {
            Parsed::Text(ref txt) => txt.to_owned(),
            Parsed::Comment(_) => "".to_owned(),
            Parsed::Mustache(ref mus) => mustache(mus, context, env)?,
            Parsed::ForEach(ref fe) => foreach(fe, context, env)?,
        })
    }
    Ok(res.join(""))
}

//pub fn mustache(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
pub fn mustache(mus: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
    expression(&mus.expr, context, env)
}


//pub fn full_expression(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
pub fn expression(expr: &FullExpression, context: &Context, env: &Incrust) -> abc::RenderResult {
    Ok(expr.filters.iter().fold(
        Ok(eval_expr(&expr.expr, context, env)?.and_then(|val| val.to_istring())),
        |result: abc::FilterResult, filter: &FilterItem| -> abc::FilterResult {
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
pub fn foreach(fe: &ForEach, context: &Context, env: &Incrust) -> abc::RenderResult {
    Ok("".into())
}

