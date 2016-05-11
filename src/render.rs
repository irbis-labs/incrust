use ::abc;
use ::context::{Context};
use ::incrust::Incrust;
use ::template::{Parsed, Mustache, ForEach, FullExpression, Expression, FilterItem, Literal};

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
    full_expression(&mus.expr, context, env)
}


//pub fn full_expression(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
pub fn full_expression(fe: &FullExpression, context: &Context, env: &Incrust) -> abc::RenderResult {
    Ok(fe.filters.iter().fold(
        Ok(expression(&fe.expr, context, env)?),
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


pub fn expression<'a>(expr: &'a Expression, context: &'a Context, env: &'a Incrust) -> abc::FilterResult {
    Ok(match *expr {
        Expression::Variable(ref id) => variable(id.as_str(), context, env)?,
        Expression::Literal(ref lit) => literal(lit, context, env)?,
    })
}

pub fn variable<'a>(id: &'a str, context: &'a Context, _: &'a Incrust) -> abc::FilterResult {
    Ok(context.get(id).map(|var| var.render()))
}

#[allow(unused_variables)]
pub fn literal<'a>(l: &'a Literal, _: &'a Context, _: &'a Incrust) -> abc::FilterResult {
    Ok(match *l {
        Literal::Char(ref c) => unimplemented!(),
        Literal::Str(ref s) => Some(s.clone()),
        Literal::Int(ref i) => unimplemented!(),
        Literal::Real(ref f) => unimplemented!(),
    })
}
