use std::borrow::Cow;
use std::fmt;

use crate::{Integer, NativeValue, Value};

impl From<bool> for Value<'static>
{
    fn from(v: bool) -> Value<'static> {
        Value::Boolean(v)
    }
}

impl NativeValue for bool {
    fn to_boolean(&self) -> Option<bool> {
        Some(*self)
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}
