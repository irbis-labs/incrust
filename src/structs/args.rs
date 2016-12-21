use std::borrow::Cow;
use std::collections::HashMap;

use abc::*;


pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, BType>;

pub fn ex<V>(v: V) -> BType where V: Into<BType> { v.into() }
