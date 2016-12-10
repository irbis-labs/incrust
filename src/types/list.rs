use super::abc::*;


impl <'b> Type for Vec<BType<'b>> {
    fn iclone<'c>(&self) -> BType<'c> {
        box self.into_iter()
            .map(|v| v.iclone())
            .collect::<Vec<BType>>()
    }
}

impl <'b> AsBool for Vec<BType<'b>> {
    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl <'b> AsIterable for Vec<BType<'b>> {
    fn try_as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl <'b> AsComposable for Vec<BType<'b>> {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


// --------------------------------------------------------------------------------------------------------------------


impl <'b> IIterable for Vec<BType<'b>> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
//    fn len(&self) -> usize {
//        Vec::len(self)
//    }
    fn ivalues(&self) -> VIterator {
        VIterator { me: self.iter() }
    }
}


impl <'b> IComposable for Vec<BType<'b>> {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as i64)),
            _ => None
        }
    }
}


impl <'a> Into<BType<'a>> for Vec<BType<'a>> {
    fn into(self) -> BType<'a> {
        box Vec::from(self)
    }
}



//impl <'a> IIterable for Vec<BType<'a>> {
//    fn ivalues<'b>(&self) -> Iterator<Item=BType<'b>> {
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
