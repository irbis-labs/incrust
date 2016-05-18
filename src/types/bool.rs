use super::abc::*;
use abc::CloneError;


impl Type for bool {
    fn to_bool(self: &Self) -> bool { *self }
}

impl ToINumeric for bool {
    fn to_real(self: &Self) -> Option<f64> { Some(if *self { 1f64 } else { 0f64 }) }
    fn to_int(self: &Self) -> Option<isize> { Some(if *self { 1isize } else { 0isize }) }
}

impl IClone for bool {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(*self) ) }
}



impl <'a> Into<BType<'a>> for bool { fn into(self) -> BType<'a> { Box::new(self) } }
