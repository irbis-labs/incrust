use super::abc::*;


impl Type for f64 {
    fn iclone(&self) -> BType {
        box *self
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

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for f64 {
    fn try_add(&self, other: BType) -> Option<BType> { other.try_as_real().map(|s| -> BType { ex(*self + s) }) }
    fn try_sub(&self, other: BType) -> Option<BType> { other.try_as_real().map(|s| -> BType { ex(*self - s) }) }
    fn try_mul(&self, other: BType) -> Option<BType> { other.try_as_real().map(|s| -> BType { ex(*self * s) }) }
    fn try_div(&self, other: BType) -> Option<BType> { other.try_as_real().map(|s| -> BType { ex(*self / s) }) }
}



impl Into<BType> for f64 {
    fn into(self) -> BType {
        box self
    }
}
