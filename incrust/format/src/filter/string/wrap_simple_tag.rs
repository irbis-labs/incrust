use crate::util::prelude::*;

pub struct WrapSimpleTag<'a, T: fmt::Display>(pub &'a str, pub T);

impl<'a, T: fmt::Display> fmt::Display for WrapSimpleTag<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (tag, content) = (&self.0, &self.1);
        write!(f, "<{tag}>{content}</{tag}>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap() {
        assert_eq!("<em>word</em>", WrapSimpleTag("em", "word").to_string());
    }
}
