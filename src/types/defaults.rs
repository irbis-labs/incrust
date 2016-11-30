use std::borrow::Cow;
use std::fmt::{Display};

use super::abc::*;


// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <T> AsString for T where T: Type {
    default fn try_as_string(&self) -> Option<Cow<str>> { None }
}

impl <T> AsString for T where T: Type + Display {
    default fn try_as_string(&self) -> Option<Cow<str>> { Some( Cow::Owned(ToString::to_string(self))) }
}

impl <T> AsReal for T where T: Type {
    default fn try_as_real(&self) -> Option<f64> { None }
}

impl <T> AsInt for T where T: Type {
    default fn try_as_int(&self) -> Option<i64> { None }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl <S> IArithm for S where S: Type {
    default fn try_add<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn try_sub<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn try_mul<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn try_div<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
}


impl <T> AsIterable for T where T: Type {
    default fn try_as_iterable(&self) -> Option<&IIterable> { None }
}

impl <T> AsComposable for T where T: Type {
    default fn try_as_composable(&self) -> Option<&IComposable> { None }
}

//impl <'a, T> AsComposable for T where T: Type + IComposable<'a> {
//    default fn try_as_composable(&self) -> Option<&IComposable> { Some(self) }
//}

impl <T> AsInvocable for T where T: Type {
    default fn try_as_invocable(&self) -> Option<&IInvocable> { None }
}

