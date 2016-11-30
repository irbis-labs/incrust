use super::abc::*;


impl Type for bool {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { *self }
}

impl AsReal for bool {
    fn try_as_real(&self) -> Option<f64> { Some(if *self { 1_f64 } else { 0_f64 }) }
}

impl AsInt for bool {
    fn try_as_int(&self) -> Option<i64> { Some(if *self { 1_i64 } else { 0_i64 }) }
}


impl <'a> Into<BType<'a>> for bool {
    fn into(self) -> BType<'a> { Box::new(self) }
}
