use std::cmp::Ordering;

use types::abc::*;
use Arg;


impl <'t> Type<'t> for f64 {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box *self)
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
    fn eq<'o>(&self, other: &'o Arg<'o>) -> bool {
        other.try_as_real().map(|s| s == *self).unwrap_or(false)
    }
}


impl IPartialOrd for f64 {
    fn partial_cmp<'o>(&self, other: &'o Arg<'o>) -> Option<Ordering> {
        if other.is_real() || other.is_int() {
            other.try_as_real().and_then(|s| (self as &PartialOrd<f64>).partial_cmp(&s))
        } else {
            None
        }
    }
}


impl IArithm for f64 {
    // todo Cow::Borrowed for Zero and One cases
    fn try_add<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_real().map(|s| { Arg::Owned(box (*self + s)) }) }
    fn try_sub<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_real().map(|s| { Arg::Owned(box (*self - s)) }) }
    fn try_mul<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_real().map(|s| { Arg::Owned(box (*self * s)) }) }
    fn try_div<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>> { other.try_as_real().map(|s| { Arg::Owned(box (*self / s)) }) }
}
