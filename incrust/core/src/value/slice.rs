use std::borrow::Cow;
use std::fmt;

use crate::value::{Integer, NativeValue, Value};

impl<'a, T> From<&'a &[T]> for Value<'a>
where
    T: NativeValue,
{
    fn from(v: &'a &[T]) -> Self {
        Value::NativeRef(v)
    }
}

impl<'a, T> NativeValue for &'a [T]
where
    T: NativeValue,
{
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn to_iterator<'s>(&'s self) -> Option<Box<dyn Iterator<Item = Value> + 's>> {
        Some(Box::new(self.iter().map(|v| Value::NativeRef(v))))
    }

    fn display(&self) -> &dyn fmt::Display {
        &"[slice]"
    }
}

impl<T> From<Box<[T]>> for Value<'static>
where
    T: NativeValue + 'static,
{
    fn from(v: Box<[T]>) -> Self {
        Value::Native(Box::new(v))
    }
}

impl<T> NativeValue for Box<[T]>
where
    T: NativeValue,
{
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn to_iterator<'s>(&'s self) -> Option<Box<dyn Iterator<Item = Value> + 's>> {
        Some(Box::new(self.iter().map(|v| Value::NativeRef(v))))
    }

    fn display(&self) -> &dyn fmt::Display {
        &"[slice]"
    }
}

impl<T> From<Vec<T>> for Value<'static>
where
    T: NativeValue + 'static,
{
    fn from(v: Vec<T>) -> Self {
        Value::from(v.into_boxed_slice())
    }
}

impl<T> NativeValue for Vec<T>
where
    T: NativeValue + 'static,
{
    fn to_boolean(&self) -> Option<bool> {
        None
    }

    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        None
    }

    fn to_iterator<'s>(&'s self) -> Option<Box<dyn Iterator<Item = Value> + 's>> {
        Some(Box::new(self.iter().map(|v| Value::NativeRef(v))))
    }

    fn display(&self) -> &dyn fmt::Display {
        &"[slice]"
    }
}
