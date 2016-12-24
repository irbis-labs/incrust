use types::abc::*;
use Arg;


impl Type for char {
    fn iclone(&self) -> Arg {
        Arg::Boxed(box *self)
    }
}
