use std::borrow::Cow;
use std::fmt::{Display};

use super::abc::*;


// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <T> AsString for T where T: Type {
    default fn as_string(&self) -> Option<Cow<str>> { None }
}

impl <T> AsString for T where T: Type + Display {
    default fn as_string(&self) -> Option<Cow<str>> { Some( Cow::Owned(ToString::to_string(self))) }
}

impl <T> AsReal for T where T: Type {
    default fn as_real(&self) -> Option<f64> { None }
}

impl <T> AsInt for T where T: Type {
    default fn as_int(&self) -> Option<isize> { None }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl <S> IArithm for S where S: Type {
    default fn iadd<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn isub<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn imul<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
    default fn idiv<'a, 'b>(&'a self, _other: BType<'a>) -> Option<BType<'b>> { None }
}


impl <T> AsIterable for T where T: Type {
    default fn as_iterable(&self) -> Option<&IIterable> { None }
}

impl <T> AsComposable for T where T: Type {
    default fn as_composable(&self) -> Option<&IComposable> { None }
}

//impl <'a, T> AsComposable for T where T: Type + IComposable<'a> {
//    default fn as_composable(&self) -> Option<&IComposable> { Some(self) }
//}

impl <T> AsInvocable for T where T: Type {
    default fn as_invocable(&self) -> Option<&IInvocable> { None }
}

