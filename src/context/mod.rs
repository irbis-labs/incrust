
use std::collections::hash_map::{HashMap, Keys, Values, Iter};

pub mod types;


pub type Args = HashMap<EntityId, Var>;


// TODO EntityId: String vs &'static str
//pub type EntityId = String;
pub type EntityId = &'static str;


pub trait Value: Send + Sync {
    fn v_render(&self) -> String;
    #[allow(unused_variables)]
    fn v_attr(&self, id: EntityId) -> Option<&Var> { None }
}


pub trait Array: Value {
    fn a_len(&self) -> usize;
    fn a_is_empty(&self) -> bool;
    fn a_values(&self) -> VIterator;
}

pub trait Map: Array {
    fn m_keys(&self) -> KIterator;
    fn m_key_values(&self) -> KVIterator;
}


pub struct VIterator<'a> {
    me: Values<'a, EntityId, Var>,
}

pub struct KIterator<'a> {
    me: Keys<'a, EntityId, Var>,
}

pub struct KVIterator<'a> {
    me: Iter<'a, EntityId, Var>,
}


pub struct Context<'a> {
    parent_scope: Option<&'a Context<'a>>,
    local_scope: &'a Args,
}

pub enum Var {
    Value(Box<Value>),
    Array(Box<Array>),
    Map(Box<Map>),
}

// --------------------------------------------------------------------------------------------------------------------

impl Var {
    pub fn ex<A:Into<Var>>(v: A) -> Var { Var::from(v.into()) }

    pub fn render(&self) -> String {
        match *self {
            Var::Value(ref v) => v.v_render(),
            Var::Array(ref v) => v.v_render(),
            Var::Map  (ref v) => v.v_render(),
        }
    }
    fn attr(&self, id: EntityId) -> Option<&Var> {
        match *self {
            Var::Value(ref v) => v.v_attr(id),
            Var::Array(ref v) => v.v_attr(id),
            Var::Map  (ref v) => v.v_attr(id),
        }
    }
    pub fn len(&self) -> usize {
        match *self {
            Var::Value(_) => 0,
            Var::Array(ref v) => v.a_len(),
            Var::Map  (ref v) => v.a_len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match *self {
            Var::Value(_) => true,
            Var::Array(ref v) => v.a_is_empty(),
            Var::Map  (ref v) => v.a_is_empty(),
        }
    }
//    pub fn as_slice(&self) -> &[&Var] {
//        match *self {
//            Var::Value(_) => &[][..],
//            Var::Array(ref v) => v.a_as_slice(),
//            Var::Map  (ref v) => v.a_as_slice(),
//        }
//    }
}

impl <'a> Iterator for VIterator<'a> {
    type Item = &'a Var;

    fn next(&mut self) -> Option<&'a Var> {
        match self.me.next() {
            Some(next) => Some(next),
            None => None
        }
    }
}

impl <'a> Iterator for KIterator<'a> {
    type Item = &'a EntityId;

    fn next(&mut self) -> Option<&'a EntityId> {
        match self.me.next() {
            Some(next) => Some(next),
            None => None
        }
    }
}

impl <'a> Iterator for KVIterator<'a> {
    type Item = (&'a EntityId, &'a Var);

    fn next(&mut self) -> Option<(&'a EntityId, &'a Var)> {
        match self.me.next() {
            Some(next) => Some(next),
            None => None
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------

impl <'a> Into<Context<'a>> for &'a Args { fn into(self) -> Context<'a> { Context::new(None, self) } }

impl <'a> Context<'a> {
    pub fn new(parent_scope: Option<&'a Context<'a>>, local_scope: &'a Args) -> Self {
        Context { parent_scope: parent_scope, local_scope: local_scope }
    }

    pub fn get(&self, id: &str) -> Option<&Var> {
        self.local_scope.get(id).or_else(|| self.parent_scope.and_then(|scope| scope.get(id)))
    }
}
