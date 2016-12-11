use std::borrow::Cow;
use std::cmp::Ordering;

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


impl IPartialEq for i64 {
    fn eq(&self, other: &BType) -> bool {
        other.try_as_int().map(|s| s == *self).unwrap_or(false)
    }
}


impl IPartialOrd for i64 {
    fn partial_cmp(&self, other: &BType) -> Option<Ordering> {
        if other.is_int() {
            other.try_as_int().and_then(|s| (self as &PartialOrd<i64>).partial_cmp(&s))
        } else {
            if other.is_real() {
                let val = *self as f64;
                other.try_as_real().and_then(|s| (&val as &PartialOrd<f64>).partial_cmp(&s))
            } else {
                None
            }
        }
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
