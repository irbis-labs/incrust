use std::borrow::Cow;

use super::abc::*;


impl Type for i64 {
    fn iclone(&self) -> BType {
        BType(box *self)
    }
}

impl AsBool for i64 {
    fn to_bool(&self) -> bool {
        *self != 0
    }
}

impl AsReal for i64 {
    fn try_as_real(&self) -> Option<f64> {
        Some(*self as f64)
    }
}

impl AsInt for i64 {
    fn try_as_int(&self) -> Option<i64> {
        Some(*self)
    }

    fn is_int(&self) -> bool {
        true
    }
}


#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for i64 {
    // todo Cow::Borrowed for Zero and One cases
    fn try_add<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { other.try_as_int().map(|s| { Cow::Owned(ex(*self + s)) }) }
    fn try_sub<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { other.try_as_int().map(|s| { Cow::Owned(ex(*self - s)) }) }
    fn try_mul<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { other.try_as_int().map(|s| { Cow::Owned(ex(*self * s)) }) }
    fn try_div<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>> { other.try_as_int().map(|s| { Cow::Owned(ex(*self / s)) }) }
}
