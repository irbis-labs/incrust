use super::abc::*;


impl Type for f64 {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { *self != 0.0 }
}

impl AsReal for f64 {
    fn as_real(&self) -> Option<f64> { Some(*self) }
}

impl AsInt for f64 {
    fn as_int(&self) -> Option<isize> { Some(*self as isize) }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for f64 {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType> { other.as_real().map(|s| -> BType { Box::new(*self + s) }) }
    fn isub(self: Box<Self>, other: BType) -> Option<BType> { other.as_real().map(|s| -> BType { Box::new(*self - s) }) }
    fn imul(self: Box<Self>, other: BType) -> Option<BType> { other.as_real().map(|s| -> BType { Box::new(*self * s) }) }
    fn idiv(self: Box<Self>, other: BType) -> Option<BType> { other.as_real().map(|s| -> BType { Box::new(*self / s) }) }
}



impl <'a> Into<BType<'a>> for f64 { fn into(self) -> BType<'a> { Box::new(self) } }
