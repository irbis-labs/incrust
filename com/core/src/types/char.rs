use crate::types::abc::*;
use crate::Arg;


impl <'t> Type<'t> for char {
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(box *self)
    }
}
