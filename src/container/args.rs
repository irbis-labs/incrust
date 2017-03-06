use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::AsRef;
use std::ops::Deref;

use types::abc::Type;


pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, Arg<'a>>;


#[inline]
pub fn ex<'r, A: Into<Arg<'r>>>(v: A) -> Arg<'r> {
    v.into()
}


#[derive(Debug)]
pub enum Arg<'r> {
    Owned(Box<for <'t> Type<'t> + 'r>),
    Ref(&'r (for <'t> Type<'t> + 'r)),
}


impl <'r> Clone for Arg<'r> {
    fn clone(&self) -> Arg<'r> {
        match *self {
            Arg::Ref(b) => Arg::Ref(b),
            Arg::Owned(ref o) => o.clone_type(),
        }
    }
}


impl <'r> Arg<'r> {
    pub fn to_ref(&'r self) -> Arg<'r> {
        Arg::Ref(self.as_ref())
    }

    pub fn to_owned(&self) -> Arg<'static> {
        self.clone_type()
    }

    pub fn into_owned(self) -> Arg<'r> {
        match self {
            Arg::Owned(_) => self,
            Arg::Ref(_) => self.clone_type(),
        }
    }
}


impl <'r> Deref for Arg<'r> {
    type Target = for <'t> Type<'t> + 'r;

    fn deref(&self) -> &Self::Target {
        match *self {
            Arg::Owned(ref b) => b.deref(),
            Arg::Ref(r) => r,
        }
    }
}


impl <'r> AsRef<for <'t> Type<'t> + 'r> for Arg<'r> {
    fn as_ref(&self) -> &(for <'t> Type<'t> + 'r) {
        &**self
    }
}


impl <'r> From<&'r Arg<'r>> for Arg<'r> {
    fn from(v: &'r Arg<'r>) -> Arg<'r> {
        Arg::Ref(v.as_ref())
    }
}

impl <'r, T: for <'t> Type<'t> + 'r> From<T> for Arg<'r> {
    fn from(v: T) -> Arg<'r> {
        Arg::Owned(box v)
    }
}

impl <'r> From<&'r (for <'t> Type<'t> + 'r)> for Arg<'r> {
    fn from(v: &'r (for <'t> Type<'t> + 'r)) -> Arg<'r> {
        Arg::Ref(v)
    }
}
