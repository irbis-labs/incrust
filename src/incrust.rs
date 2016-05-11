
use std::collections::HashMap;

use ::abc;
pub use ::context::{Args, Var, Context, EntityId};
pub use ::template::Template;


#[derive(Debug)]
pub struct Incrust {
    loaders: Vec<Box<abc::Loader>>,
    formatters: HashMap<&'static str, Box<abc::Filter>>,
}


impl Default for Incrust {
    fn default() -> Self {
        use ::formatter::{Escape, Unescape};

        let mut f: HashMap<&'static str, Box<abc::Filter>> = HashMap::new();

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
            if let Ok(template) = loader.load(name) { return Ok(template) }
        }
        Err(abc::LoadError::NotFound)
    }

    pub fn filter(&self, id: &str, value: Option<String>, context: &Context) -> abc::FilterResult {
        match self.formatters.get(&id) {
            Some(ref filter) => filter.filter(value, context, self),
            None => Err(abc::FilterError::UnknownFormatter(id.to_owned()))
        }
    }

    #[cfg_attr(feature = "clippy", allow(let_and_return))]
    pub fn parse(&self, template: &str) -> abc::ParseResult {
        let template = Template::parse(template)?;
        Ok(template)
    }

    pub fn render<'a, C:Into<Context<'a>>>(&self, name: &str, args: C) -> abc::RenderResult {
        let template = self.load(name)?;
        self.render_text(&template, args)
    }

    pub fn render_text<'a, C:Into<Context<'a>>>(&self, text: &str, args: C) -> abc::RenderResult {
        let template = self.parse(text)?;
        self.render_parsed(&template, args)
    }

    pub fn render_parsed<'a, C:Into<Context<'a>>>(&self, template: &Template, args: C) -> abc::RenderResult {
        let context: Context = args.into();
//        let mut buffer: Vec<u8> = Vec::new();
//        ::render::text(&mut buffer[..], template.parsed.as_slice(), &context, self)
        ::render::text(template.parsed.as_slice(), &context, self)
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
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, &hashmap!{ "name" => Var::ex("World") }).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn filter() {
        let templ = "<textarea>{{ html | e }}</textarea>";
        let args: Args = hashmap!{ "html" => Var::ex("<h1>Hello, World!</h1>") };
        let expected = "<textarea>&lt;h1&gt;Hello, World&#33;&lt;/h1&gt;</textarea>";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, &args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn literal() {
        let templ = r#"An escaped braces: {{ "{{" }}"#;
        let expected = "An escaped braces: {{";
        let incrust = Incrust::new();
        assert_eq!(expected, incrust.render_text(templ, &hashmap!{}).unwrap());
    }
}
