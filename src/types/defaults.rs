use std::borrow::Cow;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;

use types::abc::*;
use renderer::Writer;


// CHECK whether it is necessary
impl <'a, T> Type for &'a T where T: Type {
    fn iclone(&self) -> BType {
        (*self as &Type).iclone()
    }
}


// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <T> IRender for T where T: Type {
    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
        debug!("Default render for Type {:?}", self);
        write!(writer, "#Type")
    }
}

impl <T> IRender for T where T: Type + Display {
    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
        write!(writer, "{}", self)
    }
}

impl <T> AsString for T where T: Type {
    default fn is_string(&self) -> bool {
        false
    }

    default fn try_as_string(&self) -> Option<Cow<str>> {
        None
    }
}

impl <T> AsString for T where T: Type + Display {
    default fn try_as_string(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(ToString::to_string(self)))
    }
}

impl <T> AsBool for T where T: Type {
    default fn is_bool(&self) -> bool {
        false
    }
    default fn to_bool(&self) -> bool {
        true
    }
}

impl <T> AsReal for T where T: Type {
    default fn is_real(&self) -> bool {
        false
    }
    default fn try_as_real(&self) -> Option<f64> {
        None
    }
}

impl <T> AsInt for T where T: Type {
    default fn is_int(&self) -> bool {
        false
    }

    default fn try_as_int(&self) -> Option<i64> {
        None
    }
}

impl <T> AsIterable for T where T: Type {
    default fn is_iterable(&self) -> bool {
        self.try_as_iterable().is_some()
    }

    default fn try_as_iterable(&self) -> Option<&IIterable> {
        None
    }
}

impl <T> AsComposable for T where T: Type {
    default fn is_composable(&self) -> bool {
        false
    }

    default fn try_as_composable(&self) -> Option<&IComposable> {
        None
    }
}

impl <T> AsComposable for T where T: Type + IComposable {
    fn is_composable(&self) -> bool {
        true
    }

    default fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}

impl <T> AsInvocable for T where T: Type {
    default fn is_invocable(&self) -> bool {
        self.try_as_invocable().is_some()
    }

    default fn try_as_invocable(&self) -> Option<&IInvocable> {
        None
    }
}


impl <T> AsPartialEq for T where T: Type {
    default fn is_partial_eq(&self) -> bool {
        false
    }

    default fn try_as_partial_eq(&self) -> Option<&IPartialEq> {
        None
    }
}

impl <T> AsPartialEq for T where T: Type + IPartialEq {
    default fn is_partial_eq(&self) -> bool {
        true
    }

    default fn try_as_partial_eq(&self) -> Option<&IPartialEq> {
        Some(self)
    }
}

impl <T> AsPartialOrd for T where T: Type {
    default fn is_partial_ord(&self) -> bool {
        false
    }

    default fn try_as_partial_ord(&self) -> Option<&IPartialOrd> {
        None
    }
}

impl <T> AsPartialOrd for T where T: Type + IPartialOrd {
    default fn is_partial_ord(&self) -> bool {
        true
    }

    default fn try_as_partial_ord(&self) -> Option<&IPartialOrd> {
        Some(self)
    }
}


// -------------------------------------------------------------------------------------------------


#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl <S> IArithm for S where S: Type {
    default fn try_add<'a>(&self, _other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { None }
    default fn try_sub<'a>(&self, _other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { None }
    default fn try_mul<'a>(&self, _other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { None }
    default fn try_div<'a>(&self, _other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { None }
}
