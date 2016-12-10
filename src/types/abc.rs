use std::collections::hash_map::HashMap;
use std::borrow::Cow;
use std::fmt::Debug;
use std::iter::Iterator;
use std::slice::Iter;

use abc::EvalResult;
use incrust::Context;


// --------------------------------------------------------------------------------------------------------------------

pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, BType>;

pub fn ex<V>(v: V) -> BType where V: Into<BType> { v.into() }

// --------------------------------------------------------------------------------------------------------------------

pub type BType = Box<Type>;
pub trait Type:
    AsString + AsBool + AsReal + AsInt + AsIterable + AsComposable + AsInvocable +
    IArithm + Send + Sync + Debug
{
    fn iclone(&self) -> BType;
}


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
    fn try_as_partial_eq(&self) -> Option<&IPartialEq<T>>;
}


// --- [ impl interfaces ] --------------------------------------------------------------------------------------------

pub trait IArithm {
    fn try_add(&self, other: BType) -> Option<BType>;
    fn try_sub(&self, other: BType) -> Option<BType>;
    fn try_mul(&self, other: BType) -> Option<BType>;
    fn try_div(&self, other: BType) -> Option<BType>;
}

pub trait IInvocable: Send + Sync {
    fn invoke(&self, args: &[BType], context: &Context) -> EvalResult;
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

pub trait IPartialEq<T>: Send + Sync {
    fn eq(&self, other: &T) -> bool;
    fn ne(&self, other: &T) -> bool { !self.eq(other) }
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
