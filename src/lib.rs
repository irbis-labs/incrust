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

#![allow(clippy::items_after_statements)]
#![allow(clippy::match_bool)]

#![cfg_attr(feature = "quiet", allow(warnings))]


extern crate marksman_escape;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;
extern crate url;


pub mod abc;
pub mod container;
pub mod loader;
pub mod filter;
pub mod parser;
pub mod renderer;
pub mod types;

pub use self::abc::Loader;
pub use self::container::args::{Arg, Args, EntityId, ex};
pub use self::container::stack::{Stack, VarContext};
pub use self::container::incrust::Incrust;
pub use self::container::template::Template;
pub use self::loader::{DictLoader, FilesystemLoader, GroupLoader, NamespaceLoader};
pub use self::types::abc::Type;
pub use self::types::function::Function;
