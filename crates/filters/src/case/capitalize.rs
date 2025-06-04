use incrust_format::prelude::*;

pub struct Capitalize<T: fmt::Display>(pub T);

impl<T: fmt::Display> fmt::Display for Capitalize<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;

        let mut first = true;

        self.0.format_with(move |s: Option<&str>| {
            let Some(s) = s else { return Ok(()) };
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
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::WORD_CASES;
    use super::*;

    #[test]
    fn it_works() {
        fn test_case(expected: &str, sample: &str) {
            assert_eq!(expected, Capitalize(sample).to_string());
        }

        for &word in WORD_CASES {
            test_case("Word", word);
        }
    }

    #[test]
    fn is_works_with_dyn_display() {
        fn test_case(expected: &str, sample: &dyn fmt::Display) {
            assert_eq!(expected, Capitalize(sample).to_string());
        }

        for &word in WORD_CASES {
            test_case("Word", &word);
        }
    }
}
