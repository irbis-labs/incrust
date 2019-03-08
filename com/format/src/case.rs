use std::{
    fmt,
    fmt::Write,
};

use crate::FormatPipe;

pub struct Uppercase<T: fmt::Display>(pub T);
pub struct Lowercase<T: fmt::Display>(pub T);
pub struct Capitalize<T: fmt::Display>(pub T);

impl<T: fmt::Display> fmt::Display for Uppercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f = |s: &str| {
            for c in s.chars().flat_map(char::to_uppercase) {
                f.write_char(c)?;
            }
            Ok(())
        };
        FormatPipe(f).process(&self.0)
    }
}

impl<T: fmt::Display> fmt::Display for Lowercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f = |s: &str| {
            for c in s.chars().flat_map(char::to_lowercase) {
                f.write_char(c)?;
            }
            Ok(())
        };
        FormatPipe(f).process(&self.0)
    }
}

impl<T: fmt::Display> fmt::Display for Capitalize<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let f = |s: &str| {
            let mut i = s.chars();
            for c in i.next().into_iter().flat_map(char::to_uppercase) {
                f.write_char(c)?;
            }
            for c in i.flat_map(char::to_lowercase) {
                f.write_char(c)?;
            }
            Ok(())
        };
        FormatPipe(f).process(&self.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_upper() {
        assert_eq!("WORD", format!("{}", Uppercase("word")));
        assert_eq!("WORD", format!("{}", Uppercase("Word")));
        assert_eq!("WORD", format!("{}", Uppercase("WoRd")));
        assert_eq!("WORD", format!("{}", Uppercase("wOrD")));
        assert_eq!("WORD", format!("{}", Uppercase("WORD")));

        assert_eq!("WORD", format!("{}", Uppercase(Lowercase("WORD"))));
    }

    #[test]
    fn to_lower() {
        assert_eq!("word", format!("{}", Lowercase("word")));
        assert_eq!("word", format!("{}", Lowercase("Word")));
        assert_eq!("word", format!("{}", Lowercase("WoRd")));
        assert_eq!("word", format!("{}", Lowercase("wOrD")));
        assert_eq!("word", format!("{}", Lowercase("WORD")));

        assert_eq!("word", format!("{}", Lowercase(Uppercase("WORD"))));
    }

    #[test]
    fn capitilize() {
        assert_eq!("Word", format!("{}", Capitalize("word")));
        assert_eq!("Word", format!("{}", Capitalize("Word")));
        assert_eq!("Word", format!("{}", Capitalize("WoRd")));
        assert_eq!("Word", format!("{}", Capitalize("wOrD")));
        assert_eq!("Word", format!("{}", Capitalize("WORD")));

        assert_eq!(
            "World Wide Web",
            {
                let mut i = "world wide web".split(" ").map(Capitalize);
                format!("{} {} {}", i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
            }
        );
    }
}
