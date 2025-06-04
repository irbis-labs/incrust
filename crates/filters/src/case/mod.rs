mod capitalize;
mod lowercase;
mod uppercase;

pub use self::capitalize::*;
pub use self::lowercase::*;
pub use self::uppercase::*;

pub trait FilterCase {
    fn capitalize(self) -> Capitalize<Self>
    where
        Self: Sized + core::fmt::Display;

    fn lowercase(self) -> Lowercase<Self>
    where
        Self: Sized + core::fmt::Display;

    fn uppercase(self) -> Uppercase<Self>
    where
        Self: Sized + core::fmt::Display;
}

impl<T: core::fmt::Display> FilterCase for T {
    fn capitalize(self) -> Capitalize<Self>
    where
        Self: Sized + std::fmt::Display,
    {
        Capitalize(self)
    }

    fn lowercase(self) -> Lowercase<Self>
    where
        Self: Sized + std::fmt::Display,
    {
        Lowercase(self)
    }

    fn uppercase(self) -> Uppercase<Self>
    where
        Self: Sized + std::fmt::Display,
    {
        Uppercase(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static WORD_CASES: &[&str] = &["word", "Word", "WoRd", "wOrD", "WORD"];

    #[test]
    fn it_capitalizes() {
        fn test_case(expected: &str, sample: &str) {
            assert_eq!(expected, sample.capitalize().to_string());
        }

        for &word in WORD_CASES {
            test_case("Word", word);
        }
    }

    #[test]
    fn it_lowers_case() {
        fn test_case(expected: &str, sample: &str) {
            assert_eq!(expected, sample.lowercase().to_string());
        }

        for &word in WORD_CASES {
            test_case("word", word);
        }
    }

    #[test]
    fn it_uppers_case() {
        fn test_case(expected: &str, sample: &str) {
            assert_eq!(expected, sample.uppercase().to_string());
        }

        for &word in WORD_CASES {
            test_case("WORD", word);
        }
    }
}
