use core::fmt;

mod concat;
mod join;

pub use concat::Concat;
pub use join::Join;

pub trait DisplayIterator {
    fn display_concat(self) -> Concat<Self>
    where
        Self: Sized,
    {
        Concat::new(self)
    }

    fn display_join<S>(self, sep: S) -> Join<Self, S>
    where
        Self: Sized,
        S: fmt::Display,
    {
        Join::new(self, sep)
    }
}

impl<I> DisplayIterator for I
where
    I: IntoIterator,
    I::Item: fmt::Display,
{
    fn display_concat(self) -> Concat<Self>
    where
        Self: Sized,
    {
        Concat::new(self)
    }

    fn display_join<S>(self, sep: S) -> Join<Self, S>
    where
        Self: Sized,
        S: fmt::Display,
    {
        Join::new(self, sep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const SOURCE_STR: [&str; 4] = [
        "Sirius",
        "Canopus",
        "Rigil Kentaurus & Toliman",
        "Arcturus ",
    ];
    pub const CONCAT_SAMPLE_STR: &str = "SiriusCanopusRigil Kentaurus & TolimanArcturus ";
    pub const JOIN_SAMPLE_STR: &str = "Sirius, Canopus, Rigil Kentaurus & Toliman, Arcturus ";

    pub const SOURCE_NUM: [u32; 3] = [1, 2, 3];
    pub const CONCAT_SAMPLE_NUM: &str = "123";
    pub const JOIN_SAMPLE_NUM: &str = "1, 2, 3";

    #[test]
    fn it_concats_str() {
        assert_eq!(
            CONCAT_SAMPLE_STR,
            SOURCE_STR.iter().display_concat().to_string()
        );
    }

    #[test]
    fn it_concats_num() {
        assert_eq!(
            CONCAT_SAMPLE_NUM,
            SOURCE_NUM.iter().display_concat().to_string()
        );
    }

    #[test]
    fn it_joins_str() {
        assert_eq!(
            JOIN_SAMPLE_STR,
            SOURCE_STR.iter().display_join(", ").to_string()
        );
    }

    #[test]
    fn it_joins_num() {
        assert_eq!(
            JOIN_SAMPLE_NUM,
            SOURCE_NUM.iter().display_join(", ").to_string()
        );
    }
}
