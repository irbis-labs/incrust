use std::cmp::Ordering;

use super::abc::*;


impl Type for bool {
    fn iclone(&self) -> BType {
        BType(box *self)
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
    fn eq(&self, other: &BType) -> bool {
        other.is_bool() && *self == other.to_bool()
    }
}

impl IPartialOrd for bool {
    fn partial_cmp(&self, other: &BType) -> Option<Ordering> {
        if other.is_bool() {
            (self as &PartialOrd<bool>).partial_cmp(&other.to_bool())
        } else {
            None
        }
    }
}
