use super::abc::*;


impl Type for bool {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { *self }
}

impl ToINumeric for bool {
    fn to_real(&self) -> Option<f64> { Some(if *self { 1f64 } else { 0f64 }) }
    fn to_int(&self) -> Option<isize> { Some(if *self { 1isize } else { 0isize }) }
}


impl <'a> Into<BType<'a>> for bool { fn into(self) -> BType<'a> { Box::new(self) } }
