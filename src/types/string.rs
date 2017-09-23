use std::borrow::Cow;
use std::cmp::Ordering;

use types::abc::*;
//use types::function::Function;
use {Arg, ex};


impl <'t> Type<'t> for String {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box self.clone())
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
    fn eq<'o>(&self, other: &'o Arg<'o>) -> bool {
        other.is_string() && other.try_as_string().map(|s| s.as_ref() == self).unwrap_or(false)
    }
}


impl IPartialOrd for String {
    fn partial_cmp<'o>(&self, other: &'o Arg<'o>) -> Option<Ordering> {
        if other.is_string() {
            other.try_as_string().and_then(|s| self.as_str().partial_cmp(s.as_ref()))
        } else {
            None
        }
    }
}


impl IArithm for String {
    fn try_add<'o> (&self, other: Arg<'o>) -> Option<Arg<'o>> {
        if self == "" {
            match other.is_string() {
                true => Some(other),
                false => other.try_as_string()
                    .map(|s| ex(s.into_owned())),
            }
        } else {
            other.try_as_string()
                .map(move |s| ex(self.to_string() + s.as_ref()))
        }
    }
}


impl <'r> Into<Arg<'r>> for &'r str {
    fn into(self) -> Arg<'r> {
        ex(self.to_owned())
        // FIXME Arg::Ref(self)
    }
}


impl AsComposable for String {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


impl IComposable for String {
    fn get_attr(&self, id: &str) -> Option<Arg> {
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
//impl <'a> IInvokable for Len<'a> {
//    fn invoke(&self, _args: &[BType], _context: &Context) -> EvalResult {
//        Ok(Some((self.0.len() as i64).into()))
//    }
//}

