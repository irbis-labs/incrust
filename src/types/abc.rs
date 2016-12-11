use std::any::Any;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::hash_map::HashMap;
use std::fmt;
use std::iter::Iterator;
use std::slice::Iter;

use abc::EvalResult;
use incrust::Context;


// --------------------------------------------------------------------------------------------------------------------

pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, BType>;

pub fn ex<V>(v: V) -> BType where V: Into<BType> { v.into() }

// --------------------------------------------------------------------------------------------------------------------

//pub type BType = Box<Type>;
pub use super::btype::BType;

pub trait Type:
    AsString + AsBool + AsReal + AsInt + AsIterable + AsComposable + AsInvocable +
    AsPartialEq + AsPartialOrd +
    IArithm + IRender + fmt::Debug + Send + Sync
{
    fn iclone(&self) -> BType;
}

// --- [ try interfaces ] ---------------------------------------------------------------------------------------------

pub struct Writer<'w> (
    pub &'w mut fmt::Write
);

impl <'w> fmt::Write for Writer<'w> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

pub trait IRender {
    fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result;
}

pub trait AsAny {
    fn try_as_any(&self) -> Option<&Any>;
}

pub trait AsString {
    fn is_string(&self) -> bool;
    fn try_as_string(&self) -> Option<Cow<str>>;
}

pub trait AsBool {
    fn is_bool(&self) -> bool;
    fn to_bool(&self) -> bool;
}

pub trait AsReal {
    fn is_real(&self) -> bool;
    fn try_as_real(&self) -> Option<f64>;
}

pub trait AsInt {
    fn is_int(&self) -> bool;
    fn try_as_int(&self) -> Option<i64>;
}

pub trait AsInvocable {
    fn is_invocable(&self) -> bool;
    fn try_as_invocable(&self) -> Option<&IInvocable>;
}

pub trait AsIterable {
    fn is_iterable(&self) -> bool;
    fn try_as_iterable(&self) -> Option<&IIterable>;
}

pub trait AsComposable {
    fn is_composable(&self) -> bool;
    fn try_as_composable(&self) -> Option<&IComposable>;
}

pub trait AsIndexable {
    fn is_indexable(&self) -> bool;
    fn try_as_indexable(&self) -> Option<&IIndexable>;
}

pub trait AsPartialEq {
    fn is_partial_eq(&self) -> bool;
    fn try_as_partial_eq(&self) -> Option<&IPartialEq>;
}

pub trait AsPartialOrd {
    fn is_partial_ord(&self) -> bool;
    fn try_as_partial_ord(&self) -> Option<&IPartialOrd>;
}


// --- [ impl interfaces ] --------------------------------------------------------------------------------------------

pub trait IArithm {
    fn try_add<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>>;
    fn try_sub<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>>;
    fn try_mul<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>>;
    fn try_div<'a>(&self, other: Cow<'a, BType>) -> Option<Cow<'a, BType>>;
}

pub trait IInvocable: Send + Sync {
    fn invoke<'a: 'b, 'b>(&self, args: &'b [Cow<'a, BType>], context: &'a Context) -> EvalResult<Cow<'a, BType>>;
}

pub trait IIterable: Send + Sync {
    fn is_empty(&self) -> bool;
    fn ivalues(&self) -> VIterator;
}

pub trait IIndexable: Send + Sync {
    fn has_index(&self, index: usize) -> bool;
    fn get_index(&self, index: usize) -> Option<BType>;
//    fn as_slice(&self, range: Range) -> &[BType];
//    fn len(&self) -> usize;
}

pub trait IComposable: Send + Sync {
    fn get_attr(&self, id: &str) -> Option<BType>;
//    fn attrs(&self) -> &[BType];
}

pub trait IPartialEq: Send + Sync {
    fn eq(&self, other: &BType) -> bool;
    fn ne(&self, other: &BType) -> bool { !self.eq(other) }
}

pub trait IPartialOrd: Send + Sync {
    fn partial_cmp(&self, other: &BType) -> Option<Ordering>;
    fn lt(&self, other: &BType) -> Option<bool> {
        self.partial_cmp(other).map(|res| res == Ordering::Less)
    }
    fn le(&self, other: &BType) -> Option<bool> {
        self.partial_cmp(other).map(|res| res != Ordering::Greater)
    }
    fn gt(&self, other: &BType) -> Option<bool> {
        self.partial_cmp(other).map(|res| res == Ordering::Greater)
    }
    fn ge(&self, other: &BType) -> Option<bool> {
        self.partial_cmp(other).map(|res| res != Ordering::Less)
    }
}


// --- [ feature interfaces ] -----------------------------------------------------------------------------------------

pub struct VIterator<'a> {
    pub me: Iter<'a, BType>,
}

impl <'a> Iterator for VIterator<'a> {
    type Item = BType;

    fn next(&mut self) -> Option<BType> {
        self.me.next().map(|next| next.iclone())
    }
}
