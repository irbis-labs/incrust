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

impl <'b> AsIterable for Vec<BType<'b>> {
    fn as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl <'b> AsComposable for Vec<BType<'b>> {
    fn as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


// --------------------------------------------------------------------------------------------------------------------


impl <'a, 'b: 'a> IIterable<'a> for Vec<BType<'b>> {
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


impl <'a, 'b: 'a> IComposable<'a> for Vec<BType<'b>> {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as isize)),
            _ => None
        }
    }
}


impl <'a> Into<BType<'a>> for Vec<BType<'a>> { fn into(self) -> BType<'a> { Box::new(self) } }



//impl <'a> IIterable for Vec<BType<'a>> {
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
