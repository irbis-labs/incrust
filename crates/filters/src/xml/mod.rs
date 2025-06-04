mod cdata;

use core::fmt;

pub use cdata::*;

pub trait FilterXml {
    fn xml_c_data(self) -> XmlCData<Self>
    where
        Self: Sized + fmt::Display;
}

impl<T: fmt::Display> FilterXml for T {
    fn xml_c_data(self) -> XmlCData<Self>
    where
        Self: Sized + fmt::Display,
    {
        XmlCData(self)
    }
}
