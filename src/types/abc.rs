use std::collections::hash_map::{HashMap};
use std::borrow::Cow;
use std::fmt::{Debug};
use std::iter::Iterator;
use std::slice::Iter;

use ::abc::{EvalResult};
use ::incrust::{Context, Incrust};


// --------------------------------------------------------------------------------------------------------------------

// TODO EntityId: String vs &'static str
//pub type EntityId = String;
pub type EntityId<'a> = &'a str;

pub type Args<'a> = HashMap<Cow<'a, str>, BType<'a>>;

// --------------------------------------------------------------------------------------------------------------------

pub type BType<'a> = Box<Type + 'a>;
pub trait Type:
    AsString + AsBool + AsReal + AsInt + AsIterable + AsComposable + AsInvocable +
    IArithm + Send + Sync + Debug
{
    fn iclone<'a>(&self) -> BType<'a>;
}


#[inline]
pub fn ex<'a, A>(v: A) -> BType<'a> where A: Into<BType<'a>> { v.into() }


// --- [ try interfaces ] ---------------------------------------------------------------------------------------------

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

pub trait AsPartialEq<T> {
    fn is_partial_eq(&self) -> bool;
    fn try_as_partial_eq<'a>(&self) -> Option<&IPartialEq<'a, T>>;
}


// --- [ impl interfaces ] --------------------------------------------------------------------------------------------

pub trait IArithm {
    fn try_add<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>>;
    fn try_sub<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>>;
    fn try_mul<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>>;
    fn try_div<'a, 'b>(&'a self, other: BType<'a>) -> Option<BType<'b>>;
}

pub trait IInvocable<'a>: Send + Sync {
    fn invoke(&self, args: &[BType], context: &Context) -> EvalResult;
}

pub trait IIterable<'a>: Send + Sync {
    fn is_empty(&self) -> bool;
    fn ivalues(&self) -> VIterator;
}

pub trait IIndexable<'a>: Send + Sync {
    fn has_index(&self, index: usize) -> bool;
    fn get_index(&self, index: usize) -> Option<BType>;
//    fn as_slice(&self, range: Range) -> &[BType];
//    fn len(&self) -> usize;
}

pub trait IComposable<'a>: Send + Sync {
    fn get_attr(&self, id: &str) -> Option<BType>;
//    fn attrs(&self) -> &[BType];
}

pub trait IPartialEq<'a, T>: Send + Sync {
    fn eq(&self, other: &T) -> bool;
    fn ne(&self, other: &T) -> bool { !self.eq(other) }
}


// --- [ feature interfaces ] -----------------------------------------------------------------------------------------

pub struct VIterator<'a> {
    pub me: Iter<'a, BType<'a>>,
}

impl <'a> Iterator for VIterator<'a> {
    type Item = BType<'a>;

    fn next(&mut self) -> Option<BType<'a>> {
        self.me.next().map(|next| next.iclone())
    }
}
