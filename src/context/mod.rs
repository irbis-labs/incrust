
use std::collections::hash_map::{HashMap, Keys, Values, Iter};

pub mod types;


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


pub enum Var {
    Value(Box<Value>),
    Array(Box<Array>),
    Map(Box<Map>),
}

// --------------------------------------------------------------------------------------------------------------------

//impl <'a> Iterator for VIterator<'a> {
//    type Item = &'a Var;
//
//    fn next(&mut self) -> Option<&'a Var> {
//        match self.me.next() {
//            Some(next) => Some(next),
//            None => None
//        }
//    }
//}
//
//impl <'a> Iterator for KIterator<'a> {
//    type Item = &'a EntityId;
//
//    fn next(&mut self) -> Option<&'a EntityId> {
//        match self.me.next() {
//            Some(next) => Some(next),
//            None => None
//        }
//    }
//}
//
//impl <'a> Iterator for KVIterator<'a> {
//    type Item = (&'a EntityId, &'a Var);
//
//    fn next(&mut self) -> Option<(&'a EntityId, &'a Var)> {
//        match self.me.next() {
//            Some(next) => Some(next),
//            None => None
//        }
//    }
//}

