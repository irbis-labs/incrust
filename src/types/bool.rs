use std::cmp::Ordering;

use crate::types::abc::*;
use crate::Arg;


impl <'t> Type<'t> for bool {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box *self)
    }
}

impl AsBool for bool {
    fn is_bool(&self) -> bool {
        true
    }

    fn to_bool(&self) -> bool {
        *self
    }
}

impl AsReal for bool {
    fn try_as_real(&self) -> Option<f64> {
        Some(if *self { 1_f64 } else { 0_f64 })
    }
}

impl AsInt for bool {
    fn try_as_int(&self) -> Option<i64> {
        Some(if *self { 1_i64 } else { 0_i64 })
    }
}

impl IPartialEq for bool {
    fn eq<'o>(&self, other: &'o Arg<'o>) -> bool {
        other.is_bool() && *self == other.to_bool()
    }
}

impl IPartialOrd for bool {
    fn partial_cmp<'o>(&self, other: &'o Arg<'o>) -> Option<Ordering> {
        if other.is_bool() {
            (self as &PartialOrd<bool>).partial_cmp(&other.to_bool())
        } else {
            None
        }
    }
}
