use crate::util::prelude::*;
use crate::AbstractFilterFactory;
use crate::FilterFactory;

pub struct Lowercase<T: fmt::Display>(pub T);

pub struct LowercaseFactory;

impl<T: fmt::Display> fmt::Display for Lowercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        RevFmt::new(|s: &str| {
            for c in s.chars().flat_map(char::to_lowercase) {
                f.write_char(c)?;
            }
            Ok(())
        })
        .format(&self.0)
    }
}

impl<Input: fmt::Display> FilterFactory<Input> for LowercaseFactory {
    type Output = Lowercase<Input>;

    fn create(&self, input: Input) -> Self::Output {
        Lowercase(input)
    }
}

impl AbstractFilterFactory for LowercaseFactory {
    fn pipe<'a>(&self, input: Box<dyn fmt::Display + 'a>) -> Box<dyn fmt::Display + 'a> {
        Box::new(self.create(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_lower() {
        assert_eq!("word", Lowercase("word").to_string());
        assert_eq!("word", Lowercase("Word").to_string());
        assert_eq!("word", Lowercase("WoRd").to_string());
        assert_eq!("word", Lowercase("wOrD").to_string());
        assert_eq!("word", Lowercase("WORD").to_string());
    }
}
