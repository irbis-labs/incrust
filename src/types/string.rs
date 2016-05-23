use super::abc::*;


impl Type for String {
    fn iclone<'a>(&self) -> BType<'a> { Box::new(self.to_string()) }
    fn to_bool(&self) -> bool { !self.is_empty() }
}

#[cfg_attr(feature = "clippy", allow(boxed_local))]
impl IArithm for String {
    fn iadd(self: Box<Self>, other: BType) -> Option<BType> {
        other.to_istring().map(move |s| -> BType { Box::new(*self + s.as_str()) })
    }
}


impl <'a> Into<BType<'a>> for &'a str { fn into(self) -> BType<'a> { Box::new(self.to_owned()) } }
impl AsComposable for String { fn as_composable(&self) -> Option<&IComposable> { Some(self) } }

impl <'a> IComposable<'a> for String {
    fn get_attr(&self, id: &str) -> Option<BType> {
        match id {
            "length" => Some(ex(self.len() as isize)),
            _ => None
        }
    }
}
