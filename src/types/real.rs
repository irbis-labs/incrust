use super::abc::*;


impl Type for f64 {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { *self != 0.0 }
}

impl AsReal for f64 {
    fn try_as_real(&self) -> Option<f64> { Some(*self) }
}

impl AsInt for f64 {
    fn try_as_int(&self) -> Option<i64> { Some(*self as i64) }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for f64 {
    fn try_add<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.try_as_real().map(|s| -> BType { ex(*self + s) }) }
    fn try_sub<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.try_as_real().map(|s| -> BType { ex(*self - s) }) }
    fn try_mul<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.try_as_real().map(|s| -> BType { ex(*self * s) }) }
    fn try_div<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.try_as_real().map(|s| -> BType { ex(*self / s) }) }
}



impl <'a> Into<BType<'a>> for f64 {
    fn into(self) -> BType<'a> { Box::new(self) }
}
