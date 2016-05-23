use super::abc::*;


impl Type for char {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(*self) }
    fn to_bool(&self) -> bool { true }
}

impl <'a> Into<BType<'a>> for char { fn into(self) -> BType<'a> { Box::new(self.to_owned()) } }
