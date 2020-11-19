use std::borrow::Cow;
use std::fmt;

use super::Integer;
use super::Value;

pub trait NativeValue {
    fn to_boolean(&self) -> Option<bool>;

    fn to_integer(&self) -> Option<Cow<'_, Integer>>;

    fn to_iterator<'s>(&'s self) -> Option<Box<dyn Iterator<Item = Value> + 's>>;

    fn display(&self) -> &dyn fmt::Display;

    // fn as_any(&self) -> &dyn Any;
}

impl<'a> NativeValue for Box<dyn NativeValue + 'a> {
    fn to_boolean(&self) -> Option<bool> {
        self.as_ref().to_boolean()
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        self.as_ref().to_integer()
    }

    fn to_iterator<'s>(&'s self) -> Option<Box<dyn Iterator<Item = Value> + 's>> {
        self.as_ref().to_iterator()
    }

    fn display(&self) -> &dyn fmt::Display {
        self.as_ref().display()
    }
}
