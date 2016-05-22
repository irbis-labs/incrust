#![allow(unstable_features)]
#![feature(specialization)]

#[macro_use]
extern crate maplit;
extern crate incrust;

use std::path::Path;
use incrust::{Incrust, ex, Loader, FilesystemLoader, Type, BType};
use incrust::types::abc::{AsComposable, IComposable, IClone, CloneError};

#[derive(Debug, Clone)]
struct Fruit {
    title: String,
    price: f64,
}

impl Fruit {
    pub fn new(title: &str, price: f64) -> Fruit { Fruit { title: title.to_owned(), price: price } }
}

impl IClone for Fruit {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(self.clone()) ) }
}

impl Type for Fruit { fn to_bool(&self) -> bool { true } }
impl <'a> Into<BType<'a>> for Fruit { fn into(self) -> BType<'a> { Box::new(self) } }
impl AsComposable for Fruit { fn as_composable<'a, 'c: 'a>(&'c self) -> Option<&'a IComposable<'a>> { Some(self) } }

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

    let sample_a = sample_loader.load("2-a.html").unwrap();
    let args = || hashmap!{
        "title" => ex("fruits"),
//        "fruits" => ex(vec![ex("Orange"), ex("Apple"), ex("Banana")])
        "fruits" => ex(vec![
            ex(Fruit::new("Orange", 4.0)),
            ex(Fruit::new("Apple", 2.5)),
            ex(Fruit::new("Banana", 2.25)),
        ])
    };

    assert_eq!(sample_a, incrust.render("2", args()).unwrap());
}
