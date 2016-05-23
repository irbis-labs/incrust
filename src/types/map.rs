use std::collections::hash_map::{HashMap, Keys, Values, Iter};
use std::iter::Iterator;

use super::abc::*;


impl <'a> Type for HashMap<EntityId, BType<'a>> {
    fn to_bool(&self) -> bool { !self.is_empty() }
}

impl <'a> IClone for HashMap<EntityId, BType<'a>> {
    fn iclone<'b>(&self) -> Result<BType<'b>, CloneError> {
        let cloned: HashMap<EntityId, BType<'b>> = HashMap::new();
        for (k, v) in self.iter() {
            cloned.insert(k, v.iclone()?);
        }
        Ok( Box::new(cloned) )
    }
}


impl <'a, 'b: 'a> IIterable<'a> for HashMap<EntityId, BType<'a>> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
    //    fn len(&self) -> usize {
    //        HashMap::len(self)
    //    }
    fn ivalues(&self) -> VIterator {
        VIterator { me: self.values() }
    }
}



//impl <'a> IMap for HashMap<EntityId, BType<'a>> {
//    fn ivalues(&self) -> Option<Box<Iterator<Item=BType>>> {
//        Some(Box::new(VIterator { me: self.values() }))
//    }
////    fn ikeys(&self) -> Option<KIterator> {
////        Some(KIterator { me: self.keys() })
////    }
////    fn ikeyvalues(&self) -> Option<KVIterator> {
////        Some(KVIterator { me: self.iter() })
////    }
//}



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
