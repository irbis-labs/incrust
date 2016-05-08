
pub use ::abc;

pub struct Template {
    pub parsed: String,
}

impl Template {
    pub fn render(&self) -> abc::RenderResult {
        Ok(self.parsed.clone())
    }
}


pub fn parse(templ: &str) -> abc::ParseResult {
    let parsed = templ.to_string();
    Ok(Template { parsed: parsed })
}
