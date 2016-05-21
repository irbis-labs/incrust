#[macro_use]
extern crate maplit;
extern crate incrust;

use std::path::Path;
use incrust::{Incrust, ex, Loader, FilesystemLoader, NamespaceLoader};


#[test]
fn direct() {
    let mut incrust = Incrust::new();
    incrust.add_loader(FilesystemLoader::new(&Path::new("./assets/tpl/simple")));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("1-a.html").unwrap();
    let args = || hashmap!{
        "title" => ex("fruits"),
        "fruits" => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
    };

    assert_eq!(sample_a, incrust.render("1", args()).unwrap());
    assert_eq!(sample_a, incrust.render("1.tpl", args()).unwrap());
}

#[test]
fn with_namespace() {
    let mut incrust = Incrust::new();
    incrust.add_loader(NamespaceLoader::new("simple:", FilesystemLoader::new(&Path::new("./assets/tpl/simple"))));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("1-a.html").unwrap();
    let args = || hashmap!{
        "title" => ex("fruits"),
        "fruits" => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
    };

    assert_eq!(sample_a, incrust.render("simple:1", args()).unwrap());
    assert_eq!(sample_a, incrust.render("simple:1.tpl", args()).unwrap());
}
