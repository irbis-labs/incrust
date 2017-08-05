use std::cmp::Ordering;

use types::abc::*;
use Arg;


impl <'t> Type<'t> for i64 {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box *self)
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
    fn eq<'o>(&self, other: &'o Arg<'o>) -> bool {
        other.try_as_int().map(|s| s == *self).unwrap_or(false)
    }
}


impl IPartialOrd for i64 {
    fn partial_cmp<'o>(&self, other: &'o Arg<'o>) -> Option<Ordering> {
        if other.is_int() {
            other.try_as_int().and_then(|s| (self as &PartialOrd<i64>).partial_cmp(&s))
        } else if other.is_real() {
            let val = *self as f64;
            other.try_as_real().and_then(|s| (&val as &PartialOrd<f64>).partial_cmp(&s))
        } else {
            None
        }
    }
}


impl IArithm for i64 {
    // todo Cow::Borrowed for Zero and One cases
    fn try_add<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_int().map(|s| { Arg::Owned(box (*self + s)) }) }
    fn try_sub<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_int().map(|s| { Arg::Owned(box (*self - s)) }) }
    fn try_mul<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_int().map(|s| { Arg::Owned(box (*self * s)) }) }
    fn try_div<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_int().map(|s| { Arg::Owned(box (*self / s)) }) }
}
