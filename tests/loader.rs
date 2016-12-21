#[macro_use]
extern crate maplit;
extern crate incrust;

use std::path::Path;
use incrust::{Incrust, ex, Loader, FilesystemLoader, NamespaceLoader};


#[test]
fn dict() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(Box::new(hashmap!{
        "1".into() => r#"<h1>{{ title | e }}</h1>

<menu>
    {%- if fruits %}
    <ul>
        {%- for fruit in fruits %}
        <li>{{ loop.index }}. {{ fruit | e }}</li>
        {%- endfor %}
    </ul>
    {%- endif %}
</menu>
"#.into(),
    }));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("1-a.html").unwrap();
    let args = || hashmap!{
        "title".into() => ex("fruits"),
        "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
    };

    assert_eq!(sample_a, incrust.render("1", args()).unwrap());
}

#[test]
fn filesystem() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(FilesystemLoader::new(&Path::new("./assets/tpl/simple")));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("1-a.html").unwrap();
    let args = || hashmap!{
        "title".into() => ex("fruits"),
        "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
    };

    assert_eq!(sample_a, incrust.render("1", args()).unwrap());
    assert_eq!(sample_a, incrust.render("1.tpl", args()).unwrap());
}

#[test]
fn namespace() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(NamespaceLoader::new("simple:", FilesystemLoader::new(&Path::new("./assets/tpl/simple"))));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("1-a.html").unwrap();
    let args = || hashmap!{
        "title".into() => ex("fruits"),
        "fruits".into() => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
    };

    assert_eq!(sample_a, incrust.render("simple:1", args()).unwrap());
    assert_eq!(sample_a, incrust.render("simple:1.tpl", args()).unwrap());
}
