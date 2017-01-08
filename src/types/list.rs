use types::abc::*;
use {Arg, ex};


impl <'r, 't> Type<'t> for Vec<Arg<'r>> {
//    fn clone_type(&self) -> Arg<'static> {
//        Arg::Owned(box self.clone())
//    }
    fn clone_type(&self) -> Arg<'static> {
        Arg::Owned(
            box self.into_iter()
                .map(|v| (*v).clone_type())
                .collect::<Vec<Arg<'static>>>()
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

impl <'r> AsBool for Vec<Arg<'r>> {
    fn to_bool(&self) -> bool {
        !self.is_empty()
    }
}

impl <'r> AsIterable for Vec<Arg<'r>> {
    fn try_as_iterable(&self) -> Option<&IIterable> {
        Some(self)
    }
}

impl <'r> AsComposable for Vec<Arg<'r>> {
    fn try_as_composable(&self) -> Option<&IComposable> {
        Some(self)
    }
}


// --------------------------------------------------------------------------------------------------------------------


impl <'r> IIterable for Vec<Arg<'r>> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn ivalues(&self) -> VIterator {
        VIterator { me: self.iter() }
    }
}


impl <'r> IIndexable for Vec<Arg<'r>> {
    fn get_index(&self, index: usize) -> Option<&Arg> {
        self.get(index)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}


impl <'r> IComposable for Vec<Arg<'r>> {
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
