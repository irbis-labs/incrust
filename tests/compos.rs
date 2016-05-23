#![allow(unstable_features)]
#![feature(specialization)]

#[macro_use]
extern crate maplit;
extern crate incrust;

use std::path::Path;
use incrust::{Incrust, ex, Loader, FilesystemLoader, Type, BType, Function, Context};
use incrust::types::abc::{AsComposable, IComposable};
use incrust::abc::{EvalResult};

#[derive(Debug, Clone)]
struct Fruit {
    title: String,
    price: f64,
}

impl Fruit {
    pub fn new(title: &str, price: f64) -> Fruit { Fruit { title: title.to_owned(), price: price } }
}

impl Type for Fruit {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.clone()) }
    fn to_bool(&self) -> bool { true }
}


impl <'a> Into<BType<'a>> for Fruit { fn into(self) -> BType<'a> { Box::new(self) } }
impl AsComposable for Fruit { fn as_composable(&self) -> Option<&IComposable> { Some(self) } }

impl <'a> IComposable<'a> for Fruit {
    fn get_attr(&self, id: &str) -> Option<BType> {
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
    let args = || hashmap!{
        "title" => ex("fruits"),
        "fruits" => ex(vec![
            ex(Fruit::new("Orange", 4.0)),
            ex(Fruit::new("Apple", 2.5)),
            ex(Fruit::new("Banana", 2.25)),
        ])
    };

    assert_eq!(sample_a, incrust.render("2a", args()).unwrap());
}


#[test]
fn invocables() {
    let mut incrust = Incrust::new();
    incrust.loaders.push(FilesystemLoader::new(&Path::new("./assets/tpl/simple")));
    let sample_loader = FilesystemLoader::new(&Path::new("./assets/html/simple"));

    fn title(_: &[BType], _: &Context, _: &Incrust) -> EvalResult {
        Ok(Some(ex("fruits")))
    }

    let sample_a = sample_loader.load("2a.html").unwrap();
    let args = || hashmap!{
        "title" => Function::new(title),
        "fruits" => ex(vec![
            ex(Fruit::new("Orange", 4.0)),
            ex(Fruit::new("Apple", 2.5)),
            ex(Fruit::new("Banana", 2.25)),
        ])
    };

    assert_eq!(sample_a, incrust.render("2b", args()).unwrap());
}
