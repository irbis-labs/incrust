use core::fmt;

mod escape;

// mod unescape;

pub use self::escape::*;
// pub use self::unescape::*;

pub trait FilterUrl {
    fn url_escape(self) -> UrlEscape<Self>
    where
        Self: Sized + fmt::Display;

    // fn url_unescape(self) -> UrlUnescape<Self>
    // where
    //     Self: Sized + fmt::Display;
}

impl<T: fmt::Display> FilterUrl for T {
    fn url_escape(self) -> UrlEscape<Self>
    where
        Self: Sized + fmt::Display,
    {
        UrlEscape(self)
    }

    // fn url_unescape(self) -> UrlUnescape<Self>
    // where
    //     Self: Sized + fmt::Display,
    // {
    //     UrlUnescape(self)
    // }
}
