
use ::abc;
use ::parser;


#[derive(Debug)]
pub struct Incrust {
    loaders: Vec<Box<abc::Loader>>,
}


impl Default for Incrust {
    fn default() -> Self {
        Incrust {
            loaders: Vec::new(),
        }
    }
}

impl Incrust {
    pub fn new() -> Self { Incrust::default() }

    pub fn add_loader(&mut self, loader: Box<abc::Loader>) {
        self.loaders.push(loader);
    }

    pub fn load(&self, name: &str) -> abc::LoadResult {
        for loader in &self.loaders {
            if let Ok(template) = loader.load(&name) { return Ok(template) }
        }
        Err(abc::LoadError::NotFound)
    }

    #[cfg_attr(feature = "clippy", allow(let_and_return))]
    pub fn parse(&self, template: &str) -> abc::ParseResult {
        let template = parser::Template::parse(template)?;
        Ok(template)
    }

    #[allow(unused_variables)]
    pub fn render_text(&self, template: &str, args: abc::Args) -> abc::RenderResult {
        let template = self.parse(&template)?;
        template.render(args)
    }

    pub fn render(&self, name: &str, args: abc::Args) -> abc::RenderResult {
        let template = self.load(&name)?;
        let template = self.parse(&template)?;
        template.render(args)
    }
}


#[cfg(test)]
mod tests {
    #![allow(used_underscore_binding)]
    use super::*;

    #[test]
    fn text() {
        let templ = "Hello, World!";
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn comments() {
        let incrust = Incrust::new();
        let templ = incrust.parse("<p>Visible {# partially #} paragraph</p>").unwrap();
        let expected = "<p>Visible  paragraph</p>";
        let result = templ.render(hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn mustache() {
        let templ = "Hello, {{name}}!";
        let args = hashmap!{ "name" => "World", };
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, args).unwrap();
        assert_eq!(result, expected);
    }
}
