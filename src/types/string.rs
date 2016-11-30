use std::borrow::Cow;
use super::abc::*;
//use super::function::Function;


impl Type for String {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.to_string()) }
    fn to_bool(&self) -> bool { !self.is_empty() }
}

impl AsString for String {
    fn try_as_string(&self) -> Option<Cow<str>> { Some( Cow::Borrowed(self)) }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for String {
    fn try_add<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> {
        other.try_as_string().map(move |s| -> BType { ex(self.to_string() + s.as_ref()) })
    }
}


impl <'a> Into<BType<'a>> for String { fn into(self) -> BType<'a> { Box::new(self) } }
impl <'a> Into<BType<'a>> for &'a str { fn into(self) -> BType<'a> { Box::new(self.to_owned()) } }
impl AsComposable for String { fn try_as_composable(&self) -> Option<&IComposable> { Some(self) } }

impl <'a> IComposable<'a> for String {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as i64)),
//            "len" => Some(Function::new(|| self.len() )),
            _ => None
        }
    }
}
