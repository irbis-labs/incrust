use std::any::Any;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::iter::Iterator;
use std::slice::Iter;

use abc::EvalResult;
use renderer::Writer;
use Arg;
use Context;


// --------------------------------------------------------------------------------------------------------------------

pub trait Type<'r>:
    AsString + AsBool + AsReal + AsInt + AsIterable + AsComposable + AsInvocable +
    AsPartialEq + AsPartialOrd +
    IArithm + IRender + fmt::Debug + Send + Sync
{
    fn clone_type(&self) -> Arg<'static>;
}

// --- [ try interfaces ] ---------------------------------------------------------------------------------------------

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
    fn try_add<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>>;
    fn try_sub<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>>;
    fn try_mul<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>>;
    fn try_div<'o>(&self, other: Arg<'o>) -> Option<Arg<'o>>;
}

pub trait IInvocable {
    fn invoke<'r: 'rr, 'rr>(&self, args: &'rr [Arg<'r>], context: &'r Context<'r>) -> EvalResult<Arg<'r>>;
}

pub trait IIterable {
    fn is_empty(&self) -> bool;
    fn ivalues<'s: 'i, 'i>(&'s self) -> Box<Iterator<Item=Arg> + 'i>;
}

pub trait IIndexable {
//    fn has_index(&self, index: usize) -> bool;
    fn get_index(&self, index: usize) -> Option<Arg>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    // fn as_slice(&self) -> &[Arg];
}

pub trait IComposable {
    fn get_attr(&self, id: &str) -> Option<Arg>;
//    fn attrs(&self) -> &[BType];
}

pub trait IPartialEq {
    fn eq<'o>(&self, other: &'o Arg<'o>) -> bool;
    fn ne<'o>(&self, other: &'o Arg<'o>) -> bool { !self.eq(other) }
}

pub trait IPartialOrd {
    fn partial_cmp<'o>(&self, other: &'o Arg<'o>) -> Option<Ordering>;
    fn lt<'o>(&self, other: &'o Arg<'o>) -> Option<bool> {
        self.partial_cmp(other).map(|res| res == Ordering::Less)
    }
    fn le<'o>(&self, other: &'o Arg<'o>) -> Option<bool> {
        self.partial_cmp(other).map(|res| res != Ordering::Greater)
    }
    fn gt<'o>(&self, other: &'o Arg<'o>) -> Option<bool> {
        self.partial_cmp(other).map(|res| res == Ordering::Greater)
    }
    fn ge<'o>(&self, other: &'o Arg<'o>) -> Option<bool> {
        self.partial_cmp(other).map(|res| res != Ordering::Less)
    }
}


// --- [ feature interfaces ] -----------------------------------------------------------------------------------------

pub struct VIterator<'r> {
    pub me: Iter<'r, Arg<'r>>,
}

impl <'r> Iterator for VIterator<'r> {
    type Item = &'r Arg<'r>;

    fn next(&mut self) -> Option<Self::Item> {
        self.me.next()
    }
}
