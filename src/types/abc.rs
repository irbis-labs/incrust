use std::collections::hash_map::{HashMap};
use std::fmt::{Debug};
use std::iter::Iterator;
use std::slice::Iter;

use ::abc::{EvalResult};
use ::incrust::{Context, Incrust};


// --------------------------------------------------------------------------------------------------------------------

pub type Args<'a> = HashMap<EntityId<'a>, BType<'a>>;

// TODO EntityId: String vs &'static str
//pub type EntityId = String;
pub type EntityId<'a> = &'a str;

pub type BType<'a> = Box<Type + 'a>;
pub trait Type: ToIString + IArithm + ToINumeric + AsIterable + AsComposable + AsInvocable + Send + Sync + Debug {
    fn iclone<'a>(&self) -> BType<'a>;
    fn to_bool(&self) -> bool;
}


#[inline]
pub fn ex<'a, A>(v: A) -> BType<'a> where A: Into<BType<'a>> { v.into() }


// --- [ try interfaces ] ---------------------------------------------------------------------------------------------

pub trait ToIString {
    fn to_istring(&self) -> Option<String>;
}

pub trait ToINumeric {
    fn to_real(&self) -> Option<f64>;
    fn to_int(&self) -> Option<isize>;
}

pub trait IArithm {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType>;
    fn isub(self: Box<Self>, other: BType) -> Option<BType>;
    fn imul(self: Box<Self>, other: BType) -> Option<BType>;
    fn idiv(self: Box<Self>, other: BType) -> Option<BType>;
}

pub trait AsInvocable {
    fn as_invocable(&self) -> Option<&IInvocable>;
}

pub trait AsIterable {
    fn as_iterable(&self) -> Option<&IIterable>;
}

pub trait AsComposable {
    fn as_composable(&self) -> Option<&IComposable>;
}

pub trait AsIndexable {
    fn as_indexable(&self) -> Option<&IIndexable>;
}

pub trait AsPartialEq<T> {
    fn as_partial_eq<'a>(&self) -> Option<&IPartialEq<'a, T>>;
}


// --- [ impl interfaces ] --------------------------------------------------------------------------------------------

pub trait IInvocable<'a>: Send + Sync {
    fn invoke(&self, args: &[BType], context: &Context, env: &Incrust) -> EvalResult;
}

pub trait IIterable<'a>: Send + Sync {
    fn is_empty(&self) -> bool;
//    fn len(&self) -> usize;
    fn ivalues(&self) -> VIterator;
}

pub trait IIndexable<'a>: Send + Sync {
    fn has_index(&self, index: usize) -> bool;
    fn get_index(&self, index: usize) -> Option<BType>;
//    fn as_slice(&self, range: Range) -> &[BType];
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
