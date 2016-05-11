use ::abc;
use ::context::{Args};
use ::incrust::Incrust;
use ::template::{Parsed, Mustache, Expression, FilterItem};

pub fn text<'a>(tpl: &'a[Parsed], args: &'a Args, env: &'a Incrust) -> abc::RenderResult {
    let mut res: Vec<String> = Vec::new();
    for x in tpl {
        res.push(match *x {
            Parsed::Text(ref txt) => txt.to_owned(),
            Parsed::Comment(_) => "".to_owned(),
            Parsed::Mustache(ref mus) => mustache(mus, args, env)?,
        })
    }
    Ok(res.join(""))
}

pub fn mustache<'a>(mustache: &'a Mustache, args: &'a Args, env: &'a Incrust) -> abc::RenderResult {
    let mut value = expression(&mustache.expr, args, env)?;
    for formatter in &mustache.filters {
        value = match *formatter {
            FilterItem::Simple(ref id) => env.format(id, &value, Vec::new().as_slice())?,
        }
    }
    Ok(value)
}


#[allow(unused_variables)]
pub fn expression<'a>(expr: &'a Expression, args: &'a Args, env: &'a Incrust) -> abc::RenderResult {
    Ok(match *expr {
        Expression::Variable(ref id) => match args.get(id.as_str()) {
            Some(s) => s.render(),
            None    => "".to_owned(),
        },
    })
}
