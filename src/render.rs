use ::abc;
use ::context::{Context};
use ::incrust::Incrust;
use ::template::{Parsed, Mustache, Expression, FilterItem};

//pub fn text<'a>(buffer: &mut[u8], tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> abc::RenderResult {
pub fn text<'a>(tpl: &'a[Parsed], context: &'a Context, env: &'a Incrust) -> abc::RenderResult {
    let mut res: Vec<String> = Vec::new();
    for x in tpl {
        res.push(match *x {
            Parsed::Text(ref txt) => txt.to_owned(),
            Parsed::Comment(_) => "".to_owned(),
            Parsed::Mustache(ref mus) => mustache(mus, context, env)?,
        })
    }
    Ok(res.join(""))
}

//pub fn mustache(buffer: &mut[u8], mustache: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
pub fn mustache(mustache: &Mustache, context: &Context, env: &Incrust) -> abc::RenderResult {
    Ok(mustache.filters.iter().fold(
        Ok(expression(&mustache.expr, context, env)?),
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


#[allow(unused_variables)]
pub fn expression<'a>(expr: &'a Expression, context: &'a Context, env: &'a Incrust) -> abc::FilterResult {
    Ok(match *expr {
        Expression::Variable(ref id) => context.get(id.as_str()).map(|var| var.render()),
    })
}
