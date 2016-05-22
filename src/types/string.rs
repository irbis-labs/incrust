use super::abc::*;


impl Type for char {
    fn to_bool(self: &Self) -> bool { true }
}

impl IClone for char {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(*self) ) }
}

// -------

impl Type for String {
    fn to_bool(self: &Self) -> bool { !self.is_empty() }
}

impl IClone for String {
    fn iclone<'a>(self: &Self) -> Result<BType<'a>, CloneError> { Ok( Box::new(self.to_string()) ) }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for String {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType> {
        other.to_istring().map(move |s| -> BType { Box::new(*self + s.as_str()) })
    }
}


impl AsComposable for String {
    fn as_composable<'a, 'c: 'a>(&'c self) -> Option<&'a IComposable<'a>> { Some(self) }
}


impl <'a> IComposable<'a> for String {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as isize)),
            _ => None
        }
    }
}

impl <'a> Into<BType<'a>> for &'a str { fn into(self) -> BType<'a> { Box::new(self.to_owned()) } }
