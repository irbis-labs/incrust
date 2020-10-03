pub mod case;
pub mod escape;
pub mod iter;
pub mod wrap;

pub use self::case::*;
pub use self::escape::*;
pub use self::iter::*;
pub use self::wrap::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_words() {
        let sample = "World Wide Web";
        let source = "WORLD WIDE WEB";
        let iter = source.split(" ").map(Lowercase).map(Capitalize);

        assert_eq!(sample, Join(" ", iter).to_string());

    }
}
