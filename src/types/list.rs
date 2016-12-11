use super::abc::*;


impl Type for Vec<BType> {
    fn iclone(&self) -> BType {
        BType(
            box self.into_iter()
                .map(|v| v.0.iclone())
                .collect::<Vec<BType>>()
        )
    }
}

// todo resolve specialization conflict
//impl IRender for Vec<BType> {
//    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
//        debug!("Default render for List {:?}", self);
//        write!(writer, "#List")
//    }
//}

impl AsBool for Vec<BType> {
    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl AsIterable for Vec<BType> {
    fn try_as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl AsComposable for Vec<BType> {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


// --------------------------------------------------------------------------------------------------------------------


impl IIterable for Vec<BType> {
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


impl IComposable for Vec<BType> {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as i64)),
            _ => None
        }
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
