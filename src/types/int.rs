use super::abc::*;
use abc::CloneError;


impl Type for isize {
    fn to_bool(self: &Self) -> bool { *self != 0 }
}

impl ToINumeric for isize {
    fn to_real(self: &Self) -> Option<f64> { Some(*self as f64) }
    fn to_int(self: &Self) -> Option<isize> { Some(*self) }
}

impl IClone for isize {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(*self) ) }
}


#[allow(boxed_local)]
impl IArithm for isize {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType> { other.to_int().map(|s| -> BType { Box::new(*self + s) }) }
    fn isub(self: Box<Self>, other: BType) -> Option<BType> { other.to_int().map(|s| -> BType { Box::new(*self - s) }) }
    fn imul(self: Box<Self>, other: BType) -> Option<BType> { other.to_int().map(|s| -> BType { Box::new(*self * s) }) }
    fn idiv(self: Box<Self>, other: BType) -> Option<BType> { other.to_int().map(|s| -> BType { Box::new(*self / s) }) }
}



impl <'a> Into<BType<'a>> for isize { fn into(self) -> BType<'a> { Box::new(self) } }
