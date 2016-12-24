use std::ops::Deref;
use std::sync::Arc;

use types::abc::Type;


pub fn ex<V>(v: V) -> Arg where V: Into<Arg> {
    v.into()
}



#[derive(Clone, Debug)]
pub struct Arg (
    pub Arc<Box<Type>>
);


impl Into<Arg> for Arc<Box<Type>> {
    fn into(self) -> Arg {
        Arg(self)
    }
}

impl Into<Arg> for Box<Type> {
    fn into(self) -> Arg {
        Arg(Arc::new(self))
    }
}

impl <T> From<T> for Arg where T: Type + 'static {
    fn from(b: T) -> Self {
        Arg(Arc::new(box b))
    }
}

impl Deref for Arg {
    type Target = Arc<Box<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
