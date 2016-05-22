use std::collections::hash_map::{HashMap};
use std::fmt::{Debug};
use std::iter::Iterator;
use std::slice::Iter;


// --------------------------------------------------------------------------------------------------------------------

pub type Args<'a> = HashMap<EntityId<'a>, BType<'a>>;

// TODO EntityId: String vs &'static str
//pub type EntityId = String;
pub type EntityId<'a> = &'a str;

pub type BType<'a> = Box<Type + 'a>;
pub trait Type: ToIString + IArithm + ToINumeric + IClone + AsIterable + AsComposable + Send + Sync + Debug {
    fn to_bool(&self) -> bool;
}


#[inline]
pub fn ex<'a, A>(v: A) -> BType<'a> where A: Into<BType<'a>> { v.into() }


// --------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum CloneError {
    Error
}


// --- [ try interfaces ] ---------------------------------------------------------------------------------------------

pub trait IClone {
    fn iclone<'a>(&self) -> Result<BType<'a>, CloneError>;
}

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

pub trait AsCallable {
    fn as_callable<'a>(&self) -> Option<&ICallable<'a>>;
}

pub trait AsIterable {
    fn as_iterable<'a, 'c: 'a>(&'c self) -> Option<&'a IIterable<'a>>;
}

pub trait AsComposable {
    fn as_composable<'a, 'c: 'a>(&'c self) -> Option<&'a IComposable<'a>>;
}

pub trait AsIndexable {
    fn as_indexable<'a>(&self) -> Option<&IIndexable<'a>>;
}

pub trait AsPartialEq<T> {
    fn as_partial_eq<'a>(&self) -> Option<&IPartialEq<'a, T>>;
}


// --- [ impl interfaces ] --------------------------------------------------------------------------------------------

pub trait ICallable<'a>: Send + Sync {
    fn call(&self, args: &[BType]) -> Option<BType>;
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
    fn has_attr(&self, id: &str) -> bool;
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
        if let Some(next) = self.me.next() {
            if let Ok(next) = next.iclone() {
                return Some(next)
            }
        }
        None
    }
}
