use super::abc::*;


impl Type for char {
    fn iclone<'a>(&self) -> BType<'a> {
        box *self
    }
}

impl <'a> Into<BType<'a>> for char {
    fn into(self) -> BType<'a> {
        box self.to_owned()
    }
}
