use std::fmt;

pub struct WrapTag<'a, T: fmt::Display>(pub &'a str, pub T);

impl<'a, T: fmt::Display> fmt::Display for WrapTag<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("<")?;
        f.write_str(self.0)?;
        f.write_str(">")?;
        self.1.fmt(f)?;
        f.write_str("</")?;
        f.write_str(self.0)?;
        f.write_str(">")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap() {
        assert_eq!("<em>word</em>", WrapTag("em", "word").to_string());
    }
}
