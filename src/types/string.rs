use std::borrow::Cow;

use super::abc::*;
//use super::function::Function;


impl Type for String {
    fn iclone<'a>(&self) -> BType<'a> {
        box self.to_string()
    }
}

impl AsBool for String {
    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl AsString for String {
    fn is_string(&self) -> bool {
        true
    }

    fn try_as_string(&self) -> Option<Cow<str>> {
        Some(Cow::Borrowed(self))
    }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for String {
    fn try_add<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> {
        other.try_as_string().map(move |s| -> BType { ex(self.to_string() + s.as_ref()) })
    }
}


impl <'a> Into<BType<'a>> for String {
    fn into(self) -> BType<'a> {
        box self
    }
}

impl <'a> Into<BType<'a>> for &'a str {
    fn into(self) -> BType<'a> {
        box self.to_owned()
    }
}

impl AsComposable for String {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}

impl IComposable for String {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as i64)),
//            "len" => Some(ex(Len(self))),
//            "len" => Some(Function::new(move |_args, _context| Ok(Some(box (self.len() as i64)))).into() )),
            _ => None
        }
    }
}


//use incrust::Context;
//use abc::EvalResult;
//
//#[derive(Clone, Debug)]
//struct Len<'a>(&'a str);
//
//impl <'aa> Type for Len<'aa> {
//    fn iclone<'a>(&self) -> BType<'a> {
//        box self.clone()
//    }
//}
//
//impl <'a> Into<BType<'a>> for Len<'a> {
//    fn into(self) -> BType<'a> {
//        box self
//    }
//}
//
//impl <'a> IInvocable for Len<'a> {
//    fn invoke(&self, _args: &[BType], _context: &Context) -> EvalResult {
//        Ok(Some((self.0.len() as i64).into()))
//    }
//}

