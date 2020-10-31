use std::fmt;
use std::fmt::Write;

use crate::{FormatPipe, FilterFactory, AbstractFilterFactory};

pub struct Capitalize<T: fmt::Display>(pub T);

pub struct CapitalizeFactory;

impl<T: fmt::Display> fmt::Display for Capitalize<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        FormatPipe(|s: &str| {
            let mut i = s.chars();
            if first {
                for c in i.next().into_iter().flat_map(char::to_uppercase) {
                    first = false;
                    f.write_char(c)?;
                }
            }
            for c in i.flat_map(char::to_lowercase) {
                f.write_char(c)?;
            }
            Ok(())
        })
        .process(&self.0)?;
        Ok(())
    }
}

impl<Input: fmt::Display> FilterFactory<Input> for CapitalizeFactory {
    type Output = Capitalize<Input>;

    fn create(&self, input: Input) -> Self::Output {
        Capitalize(input)
    }
}

impl AbstractFilterFactory for CapitalizeFactory {
    fn pipe<'a>(&self, input: Box<dyn fmt::Display + 'a>) -> Box<dyn fmt::Display + 'a> {
        Box::new(self.create(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize() {
        assert_eq!("Word", Capitalize("word").to_string());
        assert_eq!("Word", Capitalize("Word").to_string());
        assert_eq!("Word", Capitalize("WoRd").to_string());
        assert_eq!("Word", Capitalize("wOrD").to_string());
        assert_eq!("Word", Capitalize("WORD").to_string());
    }
}
