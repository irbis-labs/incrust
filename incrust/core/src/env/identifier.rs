use std::borrow::Cow;
use std::ops;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    name: Cow<'static, str>,
}

impl<T> From<T> for Identifier
where
    T: Into<Cow<'static, str>>,
{
    fn from(v: T) -> Identifier {
        Identifier::new(v)
    }
}

impl Identifier {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        Identifier { name }
    }
}

impl ops::Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.name
    }
}
