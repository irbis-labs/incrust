use std::borrow::Cow;
use std::cmp::Ordering;

use super::abc::*;
//use super::function::Function;


impl Type for String {
    fn iclone(&self) -> BType {
        BType(box self.clone())
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


impl IPartialEq for String {
    fn eq(&self, other: &BType) -> bool {
        other.is_string() && other.try_as_string().map(|s| s.as_ref() == self).unwrap_or(false)
    }
}


impl IPartialOrd for String {
    fn partial_cmp(&self, other: &BType) -> Option<Ordering> {
        if other.is_string() {
            other.try_as_string().and_then(|s| self.as_str().partial_cmp(s.as_ref()))
        } else {
            None
        }
    }
}


#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for String {
    fn try_add<'a> (&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>> {
        if self == "" {
            match other.is_string() {
                true => Some(other),
                false => other.try_as_string()
                    .map(|s| Cow::Owned(ex(s.into_owned()))),
            }
        } else {
            other.try_as_string()
                .map(move |s| Cow::Owned(ex(self.to_string() + s.as_ref())))
        }
    }
}


impl <'a> Into<BType> for &'a str {
    fn into(self) -> BType {
        self.to_owned().into()
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

