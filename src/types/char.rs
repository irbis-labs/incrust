use super::abc::*;


impl Type for char {
    fn iclone(&self) -> BType {
        box *self
    }
}

impl Into<BType> for char {
    fn into(self) -> BType {
        box self.to_owned()
    }
}
