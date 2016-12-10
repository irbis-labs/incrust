use std::ops::Deref;

use super::abc::Type;


#[derive(Debug)]
pub struct BType(
    pub Box<Type>
);

impl Into<BType> for Box<Type> {
    fn into(self) -> BType {
        BType(self)
    }
}

impl <T> From<T> for BType where T: Type + 'static {
    fn from(b: T) -> Self {
        BType(box b)
    }
}

impl Deref for BType {
    type Target = Box<Type>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Clone for BType {
    fn clone(&self) -> Self {
        self.iclone()
    }
}
