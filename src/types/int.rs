use super::abc::*;


impl Type for isize {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { *self != 0 }
}

impl AsReal for isize {
    fn as_real(&self) -> Option<f64> { Some(*self as f64) }
}

impl AsInt for isize {
    fn as_int(&self) -> Option<isize> { Some(*self) }
}


#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for isize {
    fn iadd<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.as_int().map(|s| -> BType { ex(*self + s) }) }
    fn isub<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.as_int().map(|s| -> BType { ex(*self - s) }) }
    fn imul<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.as_int().map(|s| -> BType { ex(*self * s) }) }
    fn idiv<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>> { other.as_int().map(|s| -> BType { ex(*self / s) }) }
}



impl <'a> Into<BType<'a>> for isize { fn into(self) -> BType<'a> { Box::new(self) } }
