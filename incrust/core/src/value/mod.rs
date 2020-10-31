use std::borrow::Cow;
use std::fmt;

mod integer;
mod native;
mod string;

pub use self::integer::Integer;
pub use self::native::NativeValue;

pub enum Value<'a> {
    Integer(Integer),
    IntegerRef(&'a Integer),
    Display(Box<dyn fmt::Display + 'a>),
    DisplayRef(&'a (dyn fmt::Display + 'a)),
    Native(Box<dyn NativeValue + 'a>),
    NativeRef(&'a (dyn NativeValue + 'a)),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(v) => v.fmt(f),
            Value::IntegerRef(v) => v.fmt(f),
            Value::Display(v) => v.fmt(f),
            Value::DisplayRef(v) => v.fmt(f),
            Value::Native(v) => v.display().fmt(f),
            Value::NativeRef(v) => v.display().fmt(f),
        }
    }
}

impl<'a> Value<'a> {
    pub fn copy_ref(&'a self) -> Value<'a> {
        match self {
            Value::Integer(v) => match v {
                Integer::Primitive(n) => Value::Integer(Integer::Primitive(*n)),
                Integer::Big(_) => Value::IntegerRef(v),
            },
            Value::IntegerRef(v) => Value::IntegerRef(*v),
            Value::Display(v) => Value::DisplayRef(v),
            Value::DisplayRef(v) => Value::DisplayRef(*v),
            Value::Native(v) => Value::NativeRef(v),
            Value::NativeRef(v) => Value::NativeRef(*v),
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    pub fn is_display(&self) -> bool {
        matches!(self, Value::Display(_) | Value::DisplayRef(_))
    }

    pub fn is_native(&self) -> bool {
        matches!(self, Value::Native(_) | Value::NativeRef(_))
    }

    pub fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        match self {
            Value::Integer(v) => Some(Cow::Borrowed(v)),
            Value::IntegerRef(v) => Some(Cow::Borrowed(v)),
            Value::Native(v) => v.to_integer(),
            Value::NativeRef(v) => v.to_integer(),
            Value::Display(_) | Value::DisplayRef(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::{Integer, Value};

    fn test_render(sample: &str, value: Value) {
        let result = value.to_string();
        assert_eq!(sample, result)
    }

    #[test]
    fn render_integer() {
        let sample = "42";
        let primitive = Integer::Primitive(42);
        let big = Integer::Big(primitive.to_bigint().into_owned());
        test_render(sample, Value::IntegerRef(&primitive));
        test_render(sample, Value::Integer(primitive));
        test_render(sample, Value::IntegerRef(&big));
        test_render(sample, Value::Integer(big));
    }

    #[test]
    fn render_display() {
        let sample = "42";
        test_render(sample, Value::Display(Box::new("42")));
        test_render(sample, Value::DisplayRef(&"42"));
        test_render(sample, Value::Display(Box::new(42)));
        test_render(sample, Value::DisplayRef(&42));
    }

    #[test]
    fn render_native() {
        let sample = "42";
        test_render(sample, Value::Native(Box::new("42")));
        test_render(sample, Value::NativeRef(&"42"));
        test_render(sample, Value::Native(Box::new(42)));
        test_render(sample, Value::NativeRef(&42));
    }
}
