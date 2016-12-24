use std::borrow::Cow;
use std::collections::HashMap;

use Arg;


pub type EntityId<'a> = Cow<'a, str>;
pub type Args<'a> = HashMap<EntityId<'a>, Arg>;

