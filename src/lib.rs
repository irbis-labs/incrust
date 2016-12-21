// Copyright (c) 2016 Incrust developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(specialization)]

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(items_after_statements, match_bool))]

#![cfg_attr(feature = "quiet", allow(warnings))]


extern crate marksman_escape;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;


pub mod abc;
pub mod container;
pub mod loader;
pub mod parser;
pub mod renderer;
pub mod structs;
pub mod types;

pub use self::abc::Loader;
pub use self::loader::{DictLoader, FilesystemLoader, GroupLoader, NamespaceLoader};
pub use self::structs::args::{Args, EntityId, ex};
pub use self::structs::context::{GlobalContext, Context};
pub use self::structs::incrust::Incrust;
pub use self::types::abc::{Type, BType};
pub use self::types::function::Function;
pub use self::container::Template;
