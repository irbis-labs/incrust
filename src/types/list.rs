//use std::collections::hash_map::{HashMap, Keys, Values, Iter};
use std::iter::Iterator;
use std::slice::Iter;

use abc::CloneError;
use super::abc::*;


impl <'a> Type for Vec<BType<'a>> {
    fn to_bool(self: &Self) -> bool { !self.is_empty() }
}

impl <'a> IClone for Vec<BType<'a>> {
    fn iclone<'b>(self: &Self) -> Result<BType<'b>, CloneError> {
        let mut cloned: Vec<BType<'b>> = Vec::with_capacity(self.capacity());
        for v in self.iter() {
            cloned.push(v.iclone()?);
        }
        Ok( Box::new(cloned) )
    }
}

impl <'a, 'b: 'a> IIter<'a> for Vec<BType<'b>> {
    fn is_empty(self: &Self) -> bool {
        Vec::is_empty(self)
    }
//    fn len(self: &Self) -> usize {
//        Vec::len(self)
//    }
    fn ivalues(self: &Self) -> VIterator {
        VIterator { me: self.iter() }
    }
}

//impl <'a> IIter for Vec<BType<'a>> {
//    fn ivalues<'b>(self: &Self) -> Iterator<Item=BType<'b>> {
//        Some(Box::new(VIterator { me: self.iter() }))
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
