use types::abc::*;

use {BType, ex};


#[derive(Debug, Clone)]
pub struct LoopState {
    index: usize,
    is_first: bool,
    is_last: bool,
}

impl LoopState {
    pub fn new(is_last: bool) -> Self {
        LoopState {
            index: 0,
            is_first: true,
            is_last: is_last,
        }
    }

    pub fn next(self, is_last: bool) -> Self {
        LoopState {
            index: self.index + 1,
            is_first: false,
            is_last: is_last,
        }
    }
}

impl Type for LoopState {
    fn iclone(&self) -> BType {
        BType(box self.clone())
    }
}


impl IComposable for LoopState {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "index0"    => Some(ex(self.index as i64)),
            "index"     => Some(ex((self.index + 1) as i64)),
            "first"     => Some(ex(self.is_first)),
            "last"      => Some(ex(self.is_last)),
            _ => None
        }
    }
}
