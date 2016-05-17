use std::collections::hash_map::{HashMap};
use std::fmt::{Display, Debug};

use ::abc::CloneError;


// --------------------------------------------------------------------------------------------------------------------

pub type Args<'a> = HashMap<EntityId, BType<'a>>;

// TODO EntityId: String vs &'static str
//pub type EntityId = String;
pub type EntityId = &'static str;

pub type BType<'a> = Box<Type + 'a>;
pub trait Type: ToIString + IArithm + ToINumeric + IClone + Debug {
    fn to_bool(self: &Self) -> bool;
}


pub fn ex<'a, A>(v: A) -> BType<'a> where A: Into<BType<'a>> { v.into() }


// TODO enum { Box, Borrow } ?
pub struct Var;
//impl Ex for Var {
//    default fn ex<'a, A, T>(v: A) -> Box<Type + 'a> where A: Into<T>, T: Type + 'a { Box::new(v.into()) }
//}


// --- [ interfaces ] -------------------------------------------------------------------------------------------------

pub trait IClone {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError>;
}

pub trait ToIString {
    fn to_istring(self: &Self) -> Option<String>;
}

pub trait ToINumeric {
    fn to_real(self: &Self) -> Option<f64>;
    fn to_int(self: &Self) -> Option<isize>;
}

pub trait IArithm {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType>;
    fn isub(self: Box<Self>, other: BType) -> Option<BType>;
    fn imul(self: Box<Self>, other: BType) -> Option<BType>;
    fn idiv(self: Box<Self>, other: BType) -> Option<BType>;
}

// --- [ default implementations ] ------------------------------------------------------------------------------------

impl <T> IClone for T where T: Type {
    default fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Err(CloneError::Error) }
}

impl <T> ToIString for T where T: Type {
    default fn to_istring(self: &Self) -> Option<String> { None }
}

impl <T> ToIString for T where T: Type + Display {
    default fn to_istring(self: &Self) -> Option<String> { Some( <Self as ToString>::to_string(self)) }
}

impl <T> ToINumeric for T where T: Type {
    default fn to_real(self: &Self) -> Option<f64> { None }
    default fn to_int(self: &Self) -> Option<isize> { None }
}

#[allow(boxed_local)]
impl <S> IArithm for S where S: Type {
    default fn iadd(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn isub(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn imul(self: Box<Self>, _other: BType) -> Option<BType> { None }
    default fn idiv(self: Box<Self>, _other: BType) -> Option<BType> { None }
}
