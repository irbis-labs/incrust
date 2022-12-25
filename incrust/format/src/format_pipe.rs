use std::{fmt, fmt::Write};

pub struct FormatPipe<F>(pub F)
where
    F: FnMut(&str) -> fmt::Result;

impl<F> fmt::Write for FormatPipe<F>
where
    F: FnMut(&str) -> fmt::Result,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }
        (self.0)(s)
    }
}

impl<F> FormatPipe<F>
where
    F: FnMut(&str) -> fmt::Result,
{
    pub fn process<T: fmt::Display>(&mut self, s: &T) -> fmt::Result {
        write!(self, "{}", s)?;
        (self.0)("")
    }
}
