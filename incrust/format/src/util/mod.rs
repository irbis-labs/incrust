mod buffer;

pub use self::buffer::*;

pub mod prelude {
    pub use core::fmt;
    pub use core::fmt::Write;

    pub use crate::RevFmt;
}
