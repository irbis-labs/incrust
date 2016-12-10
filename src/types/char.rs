use super::abc::*;


impl Type for char {
    fn iclone(&self) -> BType {
        BType(box *self)
    }
}
