#![feature(box_syntax)]
#![feature(specialization)]

#[macro_use]
extern crate maplit;
extern crate incrust;

use std::path::Path;

use incrust::abc::EvalResult;
use incrust::{Incrust, ex, Loader, FilesystemLoader, Type, Arg, Function, VarContext};
use incrust::types::abc::{AsComposable, IComposable};

#[derive(Debug, Clone)]
struct Fruit {
    title: String,
    price: f64,
}

impl Fruit {
    pub fn new(title: &str, price: f64) -> Fruit { Fruit { title: title.to_owned(), price: price } }
}

impl <'t> Type<'t> for Fruit {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box self.clone())
    }
}


impl AsComposable for Fruit {
    fn try_as_composable(&self) -> Option<&IComposable> { Some(self) }
}


impl IComposable for Fruit {
    fn get_attr(&self, id: &str) -> Option<Arg> {
        match id {
            "title" => Some(ex(self.title.as_str())),
            "price" => Some(ex(self.price)),
            _ => None
        }
    }
}


#[test]
fn attributes() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(FilesystemLoader::new(&Path::new("./assets/tpl/simple")));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    let sample_a = sample_loader.load("2a.html").unwrap();
    let args = hashmap!{
        "title".into() => ex("fruits"),
        "fruits".into() => ex(vec![
            ex(Fruit::new("Orange", 4.0)),
            ex(Fruit::new("Apple", 2.5)),
            ex(Fruit::new("Banana", 2.25)),
        ])
    };

    assert_eq!(sample_a, incrust.render("2a", &args).unwrap());
}


#[test]
fn invokables() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(FilesystemLoader::new(&Path::new("./assets/tpl/simple")));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    fn title<'res>(_: &[Arg<'res>], _: &'res VarContext<'res>) -> EvalResult<Arg<'res>> {
        Ok(Some(ex("fruits")))
    }

    let sample_a = sample_loader.load("2a.html").unwrap();
    let args = hashmap!{
        "title".into() => Function::new(title),
        "fruits".into() => ex(vec![
            ex(Fruit::new("Orange", 4.0)),
            ex(Fruit::new("Apple", 2.5)),
            ex(Fruit::new("Banana", 2.25)),
        ])
    };

    assert_eq!(sample_a, incrust.render("2b", &args).unwrap());
}
