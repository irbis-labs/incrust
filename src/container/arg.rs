use std::ops::Deref;

use types::abc::Type;


#[derive(Debug)]
pub enum Arg {
    Boxed(Box<Type>)
}

impl Into<Arg> for Box<Type> {
    fn into(self) -> Arg {
        Arg::Boxed(self)
    }
}

impl <T> From<T> for Arg where T: Type + 'static {
    fn from(b: T) -> Self {
        Arg::Boxed(box b)
    }
}

impl Deref for Arg {
    type Target = Box<Type>;

    fn deref(&self) -> &Self::Target {
        match *self {
            Arg::Boxed(ref v) => v,
        }
    }
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
}

impl Clone for Arg {
    fn clone(&self) -> Self {
        self.iclone()
    }
}
