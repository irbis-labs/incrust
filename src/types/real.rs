use super::abc::*;
use abc::CloneError;


impl Type for f64 {
    fn to_bool(self: &Self) -> bool { *self != 0.0 }
}

impl ToINumeric for f64 {
    fn to_real(self: &Self) -> Option<f64> { Some(*self) }
    fn to_int(self: &Self) -> Option<isize> { Some(*self as isize) }
}

impl IClone for f64 {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(*self) ) }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for f64 {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType> { other.to_real().map(|s| -> BType { Box::new(*self + s) }) }
    fn isub(self: Box<Self>, other: BType) -> Option<BType> { other.to_real().map(|s| -> BType { Box::new(*self - s) }) }
    fn imul(self: Box<Self>, other: BType) -> Option<BType> { other.to_real().map(|s| -> BType { Box::new(*self * s) }) }
    fn idiv(self: Box<Self>, other: BType) -> Option<BType> { other.to_real().map(|s| -> BType { Box::new(*self / s) }) }
}



impl <'a> Into<BType<'a>> for f64 { fn into(self) -> BType<'a> { Box::new(self) } }
