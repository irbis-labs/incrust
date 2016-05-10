
use std::collections::HashMap;

use ::abc;
use ::template::Template;


#[derive(Debug)]
pub struct Incrust {
    loaders: Vec<Box<abc::Loader>>,
    formatters: HashMap<&'static str, Box<abc::Formatter>>,
}


impl Default for Incrust {
    fn default() -> Self {
        use ::formatter::{Escape, Unescape};

        let mut f: HashMap<&'static str, Box<abc::Formatter>> = HashMap::new();

        f.insert("e", Box::new(Escape));
        f.insert("escape", Box::new(Escape));
        f.insert("unescape", Box::new(Unescape));

        Incrust {
            loaders: Vec::new(),
            formatters: f,
        }
    }
}

impl Incrust {
    pub fn new() -> Self { Incrust::default() }

    pub fn no_default() -> Self {
        Incrust {
            loaders: Vec::new(),
            formatters: HashMap::new(),
        }
    }

    pub fn add_loader(&mut self, loader: Box<abc::Loader>) {
        self.loaders.push(loader);
    }

    pub fn load(&self, name: &str) -> abc::LoadResult {
        for loader in &self.loaders {
            if let Ok(template) = loader.load(&name) { return Ok(template) }
        }
        Err(abc::LoadError::NotFound)
    }

    pub fn format(&self, id: &str, value: &str, args: &[&str]) -> abc::FormatResult {
        match self.formatters.get(id) {
            Some(ref formatter) => formatter.format(value, args, &self),
            None => Err(abc::FormatError::UnknownFormatter(id.to_owned()))
        }
    }

    #[cfg_attr(feature = "clippy", allow(let_and_return))]
    pub fn parse(&self, template: &str) -> abc::ParseResult {
        let template = Template::parse(template)?;
        Ok(template)
    }

    pub fn render(&self, name: &str, args: &abc::Args) -> abc::RenderResult {
        let template = self.load(name)?;
        self.render_text(&template, args)
    }

    pub fn render_text(&self, text: &str, args: &abc::Args) -> abc::RenderResult {
        let template = self.parse(text)?;
        self.render_parsed(&template, args)
    }

    pub fn render_parsed(&self, template: &Template, args: &abc::Args) -> abc::RenderResult {
        ::render::text(template.parsed.as_slice(), args, &self)
    }
}


#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    use super::*;

    #[test]
    fn text() {
        let templ = "Hello, World!";
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, &hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn comments() {
        let incrust = Incrust::new();
        let templ = incrust.parse("<p>Visible {# partially #} paragraph</p>").unwrap();
        let expected = "<p>Visible  paragraph</p>";
        let result = incrust.render_parsed(&templ, &hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn mustache() {
        let templ = "Hello, {{name}}!";
        let args = hashmap!{ "name" => "World", };
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, &args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn filter() {
        let templ = "<textarea>{{ html | e }}</textarea>";
        let args = hashmap!{ "html" => "<h1>Hello, World!</h1>", };
         let expected = "<textarea>&lt;h1&gt;Hello, World&#33;&lt;/h1&gt;</textarea>";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, &args).unwrap();
        assert_eq!(result, expected);
    }
}
