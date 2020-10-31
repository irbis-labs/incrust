use std::borrow::Cow;
use std::fmt;

use num_bigint::BigInt;

use crate::evaluate::EvalResult;
use crate::value::{NativeValue, Value};

#[derive(Clone)]
pub enum Integer {
    Primitive(i64),
    Big(BigInt),
}

impl Integer {
    pub fn to_bigint(&self) -> Cow<'_, BigInt> {
        match self {
            Integer::Primitive(v) => Cow::Owned(BigInt::from(*v)),
            Integer::Big(v) => Cow::Borrowed(v),
        }
    }

    fn bin_op(
        &self,
        other: &Integer,
        f_prim: impl FnOnce(i64, i64) -> Option<i64>,
        f_big: impl FnOnce(BigInt, &BigInt) -> BigInt,
    ) -> EvalResult<Integer> {
        if let (Integer::Primitive(this), Integer::Primitive(other)) = (self, other) {
            if let Some(res) = f_prim(*this, *other) {
                return Ok(Integer::Primitive(res));
            }
        }
        let this = self.to_bigint().into_owned();
        let other = other.to_bigint();
        let result = f_big(this, other.as_ref());
        Ok(Integer::Big(result))
    }

    pub fn add(&self, other: &Integer) -> EvalResult<Integer> {
        self.bin_op(other, |a, b| a.checked_add(b), |a, b| a + b)
    }

    pub fn sub(&self, other: &Integer) -> EvalResult<Integer> {
        self.bin_op(other, |a, b| a.checked_sub(b), |a, b| a - b)
    }

    pub fn mul(&self, other: &Integer) -> EvalResult<Integer> {
        self.bin_op(other, |a, b| a.checked_mul(b), |a, b| a * b)
    }

    pub fn div(&self, other: &Integer) -> EvalResult<Integer> {
        self.bin_op(other, |a, b| a.checked_div(b), |a, b| a / b)
    }

    pub fn rem(&self, other: &Integer) -> EvalResult<Integer> {
        self.bin_op(other, |a, b| a.checked_rem(b), |a, b| a % b)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Integer::Primitive(v) => v.fmt(f),
            Integer::Big(v) => v.fmt(f),
        }
    }
}

impl<T> From<T> for Value<'static>
where
    Integer: From<T>,
{
    fn from(v: T) -> Value<'static> {
        Value::Integer(Integer::from(v))
    }
}

impl NativeValue for i8 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for u8 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for i16 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for u16 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for i32 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for u32 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for i64 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::from(*self)))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for u64 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::Big(BigInt::from(*self))))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for i128 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::Big(BigInt::from(*self))))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl NativeValue for u128 {
    fn to_integer(&self) -> Option<Cow<'_, Integer>> {
        Some(Cow::Owned(Integer::Big(BigInt::from(*self))))
    }

    fn display(&self) -> &dyn fmt::Display {
        self
    }
}

impl From<i8> for Integer {
    fn from(v: i8) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<u8> for Integer {
    fn from(v: u8) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<i16> for Integer {
    fn from(v: i16) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<u16> for Integer {
    fn from(v: u16) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<i32> for Integer {
    fn from(v: i32) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<u32> for Integer {
    fn from(v: u32) -> Self {
        Integer::Primitive(i64::from(v))
    }
}

impl From<i64> for Integer {
    fn from(v: i64) -> Self {
        Integer::Primitive(v)
    }
}

impl From<u64> for Integer {
    fn from(v: u64) -> Self {
        Integer::Big(BigInt::from(v))
    }
}

impl From<i128> for Integer {
    fn from(v: i128) -> Self {
        Integer::Big(BigInt::from(v))
    }
}

impl From<u128> for Integer {
    fn from(v: u128) -> Self {
        Integer::Big(BigInt::from(v))
    }
}
