use incrust_format::prelude::*;

pub struct Uppercase<T: fmt::Display>(pub T);

impl<T: fmt::Display> fmt::Display for Uppercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;

        self.0.format_with(|s: Option<&str>| {
            let Some(s) = s else { return Ok(()) };

            for c in s.chars().flat_map(char::to_uppercase) {
                f.write_char(c)?;
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::WORD_CASES;
    use super::*;

    #[test]
    fn to_upper() {
        fn test_case(expected: &str, sample: &str) {
            assert_eq!(expected, Uppercase(sample).to_string());
        }

        for &word in WORD_CASES {
            test_case("WORD", word);
        }
    }
}
