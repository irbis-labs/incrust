use std::borrow::Cow;
use std::cmp::Ordering;

use types::abc::*;
use {Arg, ex};


impl Type for f64 {
    fn iclone(&self) -> Arg {
        Arg::Boxed(box *self)
    }
}


impl AsBool for f64 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}


impl AsReal for f64 {
    fn try_as_real(&self) -> Option<f64> {
        Some(*self)
    }

    fn is_real(&self) -> bool {
        true
    }
}


impl AsInt for f64 {
    fn try_as_int(&self) -> Option<i64> {
        Some(*self as i64)
    }
}


impl IPartialEq for f64 {
    fn eq(&self, other: &Arg) -> bool {
        other.try_as_real().map(|s| s == *self).unwrap_or(false)
    }
}


impl IPartialOrd for f64 {
    fn partial_cmp(&self, other: &Arg) -> Option<Ordering> {
        if other.is_real() || other.is_int() {
            other.try_as_real().and_then(|s| (self as &PartialOrd<f64>).partial_cmp(&s))
        } else {
            None
        }
    }
}


#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for f64 {
    // todo Cow::Borrowed for Zero and One cases
    fn try_add<'a>(&self, other: Cow<'a, Arg>) -> Option<Cow<'a, Arg>> { other.try_as_real().map(|s| { Cow::Owned(ex(*self + s)) }) }
    fn try_sub<'a>(&self, other: Cow<'a, Arg>) -> Option<Cow<'a, Arg>> { other.try_as_real().map(|s| { Cow::Owned(ex(*self - s)) }) }
    fn try_mul<'a>(&self, other: Cow<'a, Arg>) -> Option<Cow<'a, Arg>> { other.try_as_real().map(|s| { Cow::Owned(ex(*self * s)) }) }
    fn try_div<'a>(&self, other: Cow<'a, Arg>) -> Option<Cow<'a, Arg>> { other.try_as_real().map(|s| { Cow::Owned(ex(*self / s)) }) }
}
