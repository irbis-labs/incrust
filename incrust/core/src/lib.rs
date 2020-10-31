// #![feature(move_ref_pattern)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]

mod args;
mod evaluate;
mod template;
mod value;

pub use self::args::*;
pub use self::evaluate::*;
pub use self::template::*;
pub use self::value::*;