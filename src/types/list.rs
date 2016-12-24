use types::abc::*;
use {Arg, ex};


impl Type for Vec<Arg> {
//    fn iclone(&self) -> Arg {
//        Arg(
//            box self.into_iter()
//                .map(|v| v.deref().iclone())
//                .collect::<Vec<Arg>>()
//        )
//    }
}

// todo resolve specialization conflict
//impl IRender for Vec<BType> {
//    default fn render<'w>(&self, writer: &mut Writer<'w>) -> fmt::Result {
//        debug!("Default render for List {:?}", self);
//        write!(writer, "#List")
//    }
//}

impl AsBool for Vec<Arg> {
    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl AsIterable for Vec<Arg> {
    fn try_as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl AsComposable for Vec<Arg> {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


// --------------------------------------------------------------------------------------------------------------------


impl IIterable for Vec<Arg> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn ivalues(&self) -> VIterator {
        VIterator { me: self.iter() }
    }
}


impl IIndexable for Vec<Arg> {
    fn get_index(&self, index: usize) -> Option<&Arg> {
        self.get(index)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}


impl IComposable for Vec<Arg> {
    fn get_attr(&self, id: &str) -> Option<Arg> {
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
