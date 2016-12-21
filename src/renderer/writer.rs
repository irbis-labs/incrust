use std::fmt;

pub struct Writer<'w> (
    pub &'w mut fmt::Write
);

impl <'w> fmt::Write for Writer<'w> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}
