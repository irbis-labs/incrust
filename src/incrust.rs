use std::borrow::Cow;
use std::collections::HashMap;

use abc::*;
pub use types::abc::{Args, Type, BType, ex};
pub use types::context::{GlobalContext, Context};
pub use container::Template;
use loader::GroupLoader;

#[derive(Debug)]
pub struct Incrust {
    pub loaders: GroupLoader,
    filters: HashMap<String, Box<Filter>>,
    top_context: HashMap<String, BType>,
}


impl Default for Incrust {
    fn default() -> Self {
        #![cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        use ::renderer::filter::{Escape, Unescape};

        let mut filters: HashMap<String, Box<Filter>> = HashMap::new();

        filters.insert("e".into(), box Escape);
        filters.insert("escape".into(), box Escape);
        filters.insert("unescape".into(), box Unescape);

        let env = hashmap!{
            "True".into()  => ex(true),
            "true".into()  => ex(true),
            "False".into() => ex(false),
            "false".into() => ex(false),
        };

        Incrust {
            loaders: Vec::new(),
            filters: filters,
            top_context: env,
        }
    }
}

impl Incrust {
    pub fn new() -> Self { Incrust::default() }

    pub fn no_default() -> Self {
        Incrust {
            loaders: Vec::new(),
            filters: HashMap::new(),
            top_context: HashMap::new(),
        }
    }

    pub fn top_context(&self) -> &HashMap<String, BType> {
        &self.top_context
    }

    pub fn load(&self, name: &str) -> LoadResult {
        for loader in &self.loaders {
            if let Ok(template) = loader.load(name) {
                return Ok(template)
            }
        }
        Err(LoadError::NotFound)
    }

    pub fn filter<'a, 's: 'a>(&'s self, id: &str, context: &'a Context, value: Option<Cow<'a, BType>>) -> FilterResult<Cow<'a, BType>> {
        match self.filters.get(id) {
            Some(ref filter) => filter.filter(context, value),
            None => Err(FilterError::UnknownFormatter(id.into()))
        }
    }

    pub fn parse(&self, template: &str) -> TemplateParseResult<Template> {
        Template::parse(template)
    }

    pub fn render<'a, C:Into<Args<'a>>>(&'a self, name: &str, args: C) -> RenderResult<String> {
        self.render_text(&self.load(name)?, args)
    }

    pub fn render_text<'a, C:Into<Args<'a>>>(&'a self, text: &str, args: C) -> RenderResult<String> {
        self.render_parsed(&self.parse(text)?, args)
    }

    pub fn render_parsed<'a, C:Into<Args<'a>>>(&'a self, template: &Template, args: C) -> RenderResult<String> {
        self.render_prepared(&self.create_global_context(template, &args.into())?.top_scope())
    }

    pub fn render_prepared(&self, context: &Context) -> RenderResult<String> {
        ::renderer::text(context)
    }

    pub fn create_global_context<'a>(&'a self, template: &'a Template, args: &'a Args<'a>) -> RenderResult<GlobalContext<'a>> {
        GlobalContext::new(self, template, args)
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
        let result = incrust.render_text(templ, hashmap!{ "name".into() => ex("World") }).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn filter() {
        let templ = "<textarea>{{ html | e }}</textarea>";
        let args: Args = hashmap!{ "html".into() => ex("<h1>Hello, World!</h1>") };
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
            "what".into() => ex("Hello"),
            "who".into() => ex("World")
        };
        assert_eq!(r#"Say: "Hello, World!""#, incrust.render_text(r#"Say: "{{ what + ", " + who }}!""#, args).unwrap());
        let args = hashmap!{
            "alpha".into() => ex(6_i64),
            "omega".into() => ex(7_f64)
        };
        assert_eq!("The answer is 42", incrust.render_text(r#"The answer is {{ alpha * omega }}"#, args).unwrap());

        let args = hashmap!{ "amount".into() => ex(6_i64) };
        assert_eq!("1 + 1 = 2", incrust.render_text(r#"1 + 1 = {{ 1 + 1 }}"#, args).unwrap());

        let args = hashmap!{ "amount".into() => ex(6_i64) };
        assert_eq!("Amount: 6 pcs", incrust.render_text(r#"Amount: {{ amount and ("" + amount + " pcs") or "-" }}"#, args).unwrap());

        let args = hashmap!{ "amount".into() => ex(0_i64) };
        assert_eq!("Amount: -", incrust.render_text(r#"Amount: {{ amount and ("" + amount + " pcs") or "-" }}"#, args).unwrap());
    }

    #[test]
    fn if_statement() {
        let incrust = Incrust::new();
        let test = |expected, template| assert_eq!(expected, incrust.render_text(template, hashmap!{}).unwrap());
        test("Mode: on",        r#"Mode: {% if True %}on{% endif %}"#);
        test("String is empty", r#"String {% if "" %}has chars{% else %}is empty{% endif %}"#);
        test("String is true",  r#"String {% if "" %}has chars{% elif True %}is true{% else %}is empty{% endif %}"#);
    }

    #[test]
    fn for_statement() {
        let incrust = Incrust::new();
        let args = hashmap!{ "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")]) };
        let tpl = r#"
    <ul>
    {%- for fruit in fruits %}
        <li>{{ index }}. {{ fruit | e }}</li>
    {%- endfor %}
    </ul>
"#;
        let expected = r#"
    <ul>
        <li>1. Orange</li>
        <li>2. Apple</li>
        <li>3. Banana</li>
    </ul>
"#;
        assert_eq!(expected, incrust.render_text(tpl, args).unwrap());
    }

    #[test]
    fn block_statement() {
        let incrust = Incrust::new();
        let args = hashmap!{ "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")]) };
        let tpl = r#"
<body>
    <h1>{% block title %}Default title{% endblock %}</h1>
</body>
"#;
        let expected = r#"
<body>
    <h1>Default title</h1>
</body>
"#;
        assert_eq!(expected, incrust.render_text(tpl, args).unwrap());
    }

    #[test]
    fn inheritance() {
        let base = r#"
<body>
    <h1>{% block title %}Default title{% endblock %}</h1>
    <main>
    {%- block body %}
        <p>Default body<p>
    {%- endblock %}
    </main>
</body>
"#;
        let tpl = r#"
{% extends parent_layout %}
{% block title -%}
    New title
{%- endblock %}
"#;

        let expected = r#"
<body>
    <h1>New title</h1>
    <main>
        <p>Default body<p>
    </main>
</body>
"#;

        let mut incrust = Incrust::new();
        incrust.loaders.push(box hashmap!{
            "base".into() => base.into(),
            "tpl".into() => tpl.into(),
        });

        let args = hashmap!{ "parent_layout".into() => ex("base") };
        assert_eq!(expected, incrust.render("tpl", args).unwrap());
    }

    #[test]
    fn include() {
        let default_menu = r#"
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/about">About Us</a></li>
    </ul>
"#;
        let tpl = r#"
<nav>
    {%- include menu -%}
</nav>

<h1>Body</h1>
"#;

        let expected = r#"
<nav>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/about">About Us</a></li>
    </ul>
</nav>

<h1>Body</h1>
"#;

        let mut incrust = Incrust::new();
        incrust.loaders.push(box hashmap!{
            "default_menu".into() => default_menu.into(),
            "tpl".into() => tpl.into(),
        });

        let args = hashmap!{ "menu".into() => ex("default_menu") };
        assert_eq!(expected, incrust.render("tpl", args).unwrap());
    }
}
