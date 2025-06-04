pub use smart_string::DisplayExt;

pub mod iter;
pub mod str_buffer;

pub mod prelude {
    pub use core::fmt;

    pub use smart_string::DisplayExt;
    pub use smart_string::PascalString;

    pub use crate::iter::DisplayIterator;
    pub use crate::str_buffer::StrBuffer;
}
