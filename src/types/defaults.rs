use std::borrow::Cow;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;

use types::abc::*;
use renderer::Writer;
use Arg;


// CHECK whether it is necessary
//impl <'a, T> Type for &'a T where T: Type {
////    fn iclone(&self) -> Arg {
////        (*self as &Type).iclone()
////    }
//}


// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <'r, T> IRender for T where T: for <'t> Type<'t> + 'r {
    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
        debug!("Default render for Type {:?}", self);
        write!(writer, "#Type")
    }
}

impl <'r, T> IRender for T where T: for <'t> Type<'t> + 'r + Display {
    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
        write!(writer, "{}", self)
    }
}

impl <'r, T> AsString for T where T: for <'t> Type<'t> + 'r {
    default fn is_string(&self) -> bool {
        false
    }

    default fn try_as_string(&self) -> Option<Cow<str>> {
        None
    }
}

impl <'r, T> AsString for T where T: for <'t> Type<'t> + 'r + Display {
    default fn try_as_string(&self) -> Option<Cow<str>> {
        Some(Cow::Owned(ToString::to_string(self)))
    }
}

impl <'r, T> AsBool for T where T: for <'t> Type<'t> + 'r {
    default fn is_bool(&self) -> bool {
        false
    }
    default fn to_bool(&self) -> bool {
        true
    }
}

impl <'r, T> AsReal for T where T: for <'t> Type<'t> + 'r {
    default fn is_real(&self) -> bool {
        false
    }
    default fn try_as_real(&self) -> Option<f64> {
        None
    }
}

impl <'r, T> AsInt for T where T: for <'t> Type<'t> + 'r {
    default fn is_int(&self) -> bool {
        false
    }

    default fn try_as_int(&self) -> Option<i64> {
        None
    }
}

impl <'r, T> AsIterable for T where T: for <'t> Type<'t> + 'r {
    default fn is_iterable(&self) -> bool {
        false
    }

    default fn try_as_iterable(&self) -> Option<&IIterable> {
        None
    }
}

impl <'r, T> AsIterable for T where T: for <'t> Type<'t> + 'r + IIterable {
    fn is_iterable(&self) -> bool {
        true
    }

    default fn try_as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl <'r, T> AsIndexable for T where T: for <'t> Type<'t> + 'r {
    default fn is_indexable(&self) -> bool {
        false
    }

    default fn try_as_indexable(&self) -> Option<&IIndexable> {
        None
    }
}

impl <'r, T> AsIndexable for T where T: for <'t> Type<'t> + 'r + IIndexable {
    fn is_indexable(&self) -> bool {
        true
    }

    default fn try_as_indexable(&self) -> Option<&IIndexable> {
        Some(self)
    }
}

impl <'r, T> AsMappable for T where T: for <'t> Type<'t> + 'r {
    default fn is_mappable(&self) -> bool {
        false
    }

    default fn try_as_mappable(&self) -> Option<&IMappable> {
        None
    }
}

impl <'r, T> AsMappable for T where T: for <'t> Type<'t> + 'r + IMappable {
    fn is_mappable(&self) -> bool {
        true
    }

    default fn try_as_mappable(&self) -> Option<&IMappable> {
        Some(self)
    }
}

impl <'r, T> AsComposable for T where T: for <'t> Type<'t> + 'r {
    default fn is_composable(&self) -> bool {
        false
    }

    default fn try_as_composable(&self) -> Option<&IComposable> {
        None
    }
}

impl <'r, T> AsComposable for T where T: for <'t> Type<'t> + 'r + IComposable {
    fn is_composable(&self) -> bool {
        true
    }

    default fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}

impl <'r, T> AsInvocable for T where T: for <'t> Type<'t> + 'r {
    default fn is_invocable(&self) -> bool {
        false
    }

    default fn try_as_invocable(&self) -> Option<&IInvocable> {
        None
    }
}

impl <'r, T> AsInvocable for T where T: for <'t> Type<'t> + 'r + IInvocable {
    fn is_invocable(&self) -> bool {
        true
    }

    default fn try_as_invocable(&self) -> Option<&IInvocable> {
        Some(self)
    }
}


impl <'r, T> AsPartialEq for T where T: for <'t> Type<'t> + 'r {
    default fn is_partial_eq(&self) -> bool {
        false
    }

    default fn try_as_partial_eq(&self) -> Option<&IPartialEq> {
        None
    }
}

impl <'r, T> AsPartialEq for T where T: for <'t> Type<'t> + 'r + IPartialEq {
    default fn is_partial_eq(&self) -> bool {
        true
    }

    default fn try_as_partial_eq(&self) -> Option<&IPartialEq> {
        Some(self)
    }
}

impl <'r, T> AsPartialOrd for T where T: for <'t> Type<'t> + 'r {
    default fn is_partial_ord(&self) -> bool {
        false
    }

    default fn try_as_partial_ord(&self) -> Option<&IPartialOrd> {
        None
    }
}

impl <'r, T> AsPartialOrd for T where T: for <'t> Type<'t> + 'r + IPartialOrd {
    default fn is_partial_ord(&self) -> bool {
        true
    }

    default fn try_as_partial_ord(&self) -> Option<&IPartialOrd> {
        Some(self)
    }
}


// -------------------------------------------------------------------------------------------------


impl <'r, S> IArithm for S where S: for <'t> Type<'t> + 'r {
    default fn try_add<'o>(&self, _other: Arg<'o>) -> Option<Arg<'o>> { None }
    default fn try_sub<'o>(&self, _other: Arg<'o>) -> Option<Arg<'o>> { None }
    default fn try_mul<'o>(&self, _other: Arg<'o>) -> Option<Arg<'o>> { None }
    default fn try_div<'o>(&self, _other: Arg<'o>) -> Option<Arg<'o>> { None }
}
