use std::fmt;
use std::fmt::Write;

use crate::{AbstractFilterFactory, FilterFactory, FormatPipe};

pub struct Uppercase<T: fmt::Display>(pub T);

pub struct UppercaseFactory;

impl<T: fmt::Display> fmt::Display for Uppercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        FormatPipe(|s: &str| {
            for c in s.chars().flat_map(char::to_uppercase) {
                f.write_char(c)?;
            }
            Ok(())
        })
        .process(&self.0)
    }
}

impl<Input: fmt::Display> FilterFactory<Input> for UppercaseFactory {
    type Output = Uppercase<Input>;

    fn create(&self, input: Input) -> Self::Output {
        Uppercase(input)
    }
}

impl AbstractFilterFactory for UppercaseFactory {
    fn pipe<'a>(&self, input: Box<dyn fmt::Display + 'a>) -> Box<dyn fmt::Display + 'a> {
        Box::new(self.create(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_upper() {
        assert_eq!("WORD", Uppercase("word").to_string());
        assert_eq!("WORD", Uppercase("Word").to_string());
        assert_eq!("WORD", Uppercase("WoRd").to_string());
        assert_eq!("WORD", Uppercase("wOrD").to_string());
        assert_eq!("WORD", Uppercase("WORD").to_string());
    }
}
