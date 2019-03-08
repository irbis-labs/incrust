use std::fmt;

pub struct WrapTag<'a, T: fmt::Display>(pub &'a str, pub T);

impl<'a, T: fmt::Display> fmt::Display for WrapTag<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("<")?;
        f.write_str(self.0)?;
        f.write_str(">")?;
        write!(f, "{}", self.1)?;
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
    fn to_upper() {
        assert_eq!("<em>word</em>", format!("{}", WrapTag("em", "word")));
    }
}