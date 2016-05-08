
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

    pub fn load_template(&self, name: &str) -> abc::LoadResult {
        for loader in &self.loaders {
            if let Ok(template) = loader.load(&name) { return Ok(template) }
        }
        Err(abc::LoadError::NotFound)
    }

    #[allow(let_and_return)]
    pub fn parse_template(&self, template: &str) -> abc::ParseResult {
        let template = parser::parse(template)?;
        Ok(template)
    }

    pub fn render_text(&self, template: &str) -> abc::RenderResult {
        let template = self.parse_template(&template)?;
        template.render()
    }

    pub fn render_template(&self, name: &str) -> abc::RenderResult {
        let template = self.load_template(&name)?;
        let template = self.parse_template(&template)?;
        template.render()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text() {
        let templ = "Hello, World!";
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ).unwrap();
        assert_eq!(result, expected);
    }

//    #[test]
//    fn mustache() {
//        use std::collections::HashMap;
//        use std::iter::FromIterator;
//
//        let templ = "Hello, {{name}}!";
//        let name = "World";
//        let args: HashMap<&str, &str> = HashMap::from_iter(vec![("name", "World")]);
//        let expected = "Hello, World!";
//        let incrust = Incrust::new();
//        let result = incrust.render_text(templ, args).unwrap();
//        assert_eq!(result, expected);
//    }
}
