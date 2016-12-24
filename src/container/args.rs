use std::borrow::Cow;
use std::collections::HashMap;

use Arg;


pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, Arg>;

pub fn ex<V>(v: V) -> Arg where V: Into<Arg> { v.into() }
