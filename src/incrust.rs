
use std::collections::HashMap;

use ::abc;
pub use ::types::abc::{Args, EntityId, Type, BType, ex};
pub use ::types::context::{Context};
pub use ::template::Template;
use ::loader::GroupLoader;

#[derive(Debug)]
pub struct Incrust<'a> {
    pub loaders: GroupLoader,
    formatters: HashMap<&'a str, Box<abc::Filter>>,
    env_context: Args<'a>,
}


impl <'a> Default for Incrust<'a> {
    fn default() -> Self {
        #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        use ::renderer::filter::{Escape, Unescape};

        let mut f: HashMap<&'static str, Box<abc::Filter>> = HashMap::new();

        f.insert("e", Box::new(Escape));
        f.insert("escape", Box::new(Escape));
        f.insert("unescape", Box::new(Unescape));

        let env: Args = hashmap!{
            "True"  => ex(true),
            "true"  => ex(true),
            "False" => ex(false),
            "false" => ex(false),
        };

        Incrust {
            loaders: Vec::new(),
            formatters: f,
            env_context: env,
        }
    }
}

impl <'aa> Incrust<'aa> {
    pub fn new() -> Self { Incrust::default() }

    pub fn no_default() -> Self {
        Incrust {
            loaders: Vec::new(),
            formatters: HashMap::new(),
            env_context: HashMap::new(),
        }
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

    pub fn render<'a, C:Into<Args<'a>>>(&self, name: &str, args: C) -> abc::RenderResult {
        let template = self.load(name)?;
        self.render_text(&template, args)
    }

    pub fn render_text<'a, C:Into<Args<'a>>>(&self, text: &str, args: C) -> abc::RenderResult {
        let template = self.parse(text)?;
        self.render_parsed(&template, args)
    }

    pub fn render_parsed<'a, C:Into<Args<'a>>>(&self, template: &Template, args: C) -> abc::RenderResult {
        let args: Args = args.into();
        let env = Context::new(None, &self.env_context);
        let context = Context::new(Some(&env), &args);
        self.render_prepared(template, &context)
    }

    pub fn render_prepared(&self, template: &Template, context: &Context) -> abc::RenderResult {
//        let mut buffer: Vec<u8> = Vec::new();
//        ::render::text(&mut buffer[..], template.parsed.as_slice(), &context, self)
        ::renderer::text(template.parsed.as_slice(), &context, self)
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
        let result = incrust.render_text(templ, hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn comments() {
        let incrust = Incrust::new();
        let templ = incrust.parse("<p>Visible {# partially #} paragraph</p>").unwrap();
        let expected = "<p>Visible  paragraph</p>";
        let result = incrust.render_parsed(&templ, hashmap!{}).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn mustache() {
        let templ = "Hello, {{name}}!";
        let expected = "Hello, World!";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, hashmap!{ "name" => ex("World") }).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn filter() {
        let templ = "<textarea>{{ html | e }}</textarea>";
        let args: Args = hashmap!{ "html" => ex("<h1>Hello, World!</h1>") };
        let expected = "<textarea>&lt;h1&gt;Hello, World&#33;&lt;/h1&gt;</textarea>";
        let incrust = Incrust::new();
        let result = incrust.render_text(templ, args).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn literal() {
        let incrust = Incrust::new();
        assert_eq!("Braces: {{", incrust.render_text(r#"Braces: {{ "{{" }}"#, hashmap!{}).unwrap());
        assert_eq!("The answer: 42", incrust.render_text(r#"The answer: {{ 42 }}"#, hashmap!{}).unwrap());
        assert_eq!("Pi: 3.1415926", incrust.render_text(r#"Pi: {{ 3.1415926 }}"#, hashmap!{}).unwrap());
    }

    #[test]
    fn expression() {
        let incrust = Incrust::new();
        let args = hashmap!{
            "what" => ex("Hello"),
            "who" => ex("World")
        };
        assert_eq!(r#"Say: "Hello, World!""#, incrust.render_text(r#"Say: "{{ what + ", " + who }}!""#, args).unwrap());
        let args = hashmap!{
            "alpha" => ex(6isize),
            "omega" => ex(7f64)
        };
        assert_eq!("The answer is 42", incrust.render_text(r#"The answer is {{ alpha * omega }}"#, args).unwrap());

        let args = hashmap!{ "amount" => ex(6isize) };
        assert_eq!("Amount: 6 pcs", incrust.render_text(r#"Amount: {{ amount and ("" + amount + " pcs") or "-" }}"#, args).unwrap());

        let args = hashmap!{ "amount" => ex(0isize) };
        assert_eq!("Amount: -", incrust.render_text(r#"Amount: {{ amount and ("" + amount + " pcs") or "-" }}"#, args).unwrap());
    }

    #[test]
    fn if_statement() {
        let incrust = Incrust::new();
        assert_eq!("Mode: on", incrust.render_text(r#"Mode: {% if True %}on{% endif %}"#, hashmap!{}).unwrap());
        assert_eq!("String is empty", incrust.render_text(r#"String {% if "" %}has chars{% else %}is empty{% endif %}"#, hashmap!{}).unwrap());
        assert_eq!("String is true", incrust.render_text(r#"String {% if "" %}has chars{% elif True %}is true{% else %}is empty{% endif %}"#, hashmap!{}).unwrap());
    }

    #[test]
    fn for_statement() {
        let incrust = Incrust::new();
        let args = hashmap!{ "fruits" => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")]) };
        let tpl = r#"<ul>{% for fruit in fruits %}<li>{{ index }}. {{ fruit | e }}</li>{% endfor %}</ul>"#;
        let expected = r#"<ul><li>1. Orange</li><li>2. Apple</li><li>3. Banana</li></ul>"#;
        assert_eq!(expected, incrust.render_text(tpl, args).unwrap());
    }
}
