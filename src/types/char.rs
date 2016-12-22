use types::abc::*;
use BType;


impl Type for char {
    fn iclone(&self) -> BType {
        BType(box *self)
    }
}
