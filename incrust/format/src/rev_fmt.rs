use crate::util::prelude::*;

pub struct RevFmt<F> {
    fmt: Fmt<F>,
}

struct Fmt<F>(F);

impl<F> fmt::Write for Fmt<F>
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

impl<F> RevFmt<F>
where
    F: FnMut(&str) -> fmt::Result,
{
    pub fn new(f: F) -> Self {
        RevFmt { fmt: Fmt(f) }
    }

    pub fn format(&mut self, s: impl fmt::Display) -> fmt::Result {
        write!(&mut self.fmt, "{}", s)?;
        (self.fmt.0)("")
    }
}
