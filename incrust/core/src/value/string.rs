use std::borrow::Cow;
use std::fmt;

use crate::value::{Integer, NativeValue, Value};

impl<'a> From<&'a &str> for Value<'a> {
    fn from(v: &'a &str) -> Self {
        Value::NativeRef(v)
    }
}

impl<'a> NativeValue for &'a str {
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl From<Box<str>> for Value<'static> {
    fn from(v: Box<str>) -> Self {
        Value::Native(Box::new(v))
    }
}

impl NativeValue for Box<str> {
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn display(&self) -> &dyn fmt::Display {
        &*self
    }
}

impl From<String> for Value<'static> {
    fn from(v: String) -> Self {
        Value::from(v.into_boxed_str())
    }
}

impl NativeValue for String {
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn display(&self) -> &dyn fmt::Display {
        &*self
    }
}
