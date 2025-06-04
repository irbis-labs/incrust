use crate::prelude::*;

pub struct Concat<T> {
    pub iter: T,
}

impl<T> Concat<T> {
    pub fn new(iter: T) -> Self {
        Concat { iter }
    }
}

impl<T> fmt::Display for Concat<T>
where
    T: Iterator + Clone,
    T::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.iter.clone() {
            item.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::CONCAT_SAMPLE_NUM;
    use super::super::tests::CONCAT_SAMPLE_STR;
    use super::super::tests::SOURCE_NUM;
    use super::super::tests::SOURCE_STR;
    use super::*;

    #[test]
    fn it_concats_str() {
        assert_eq!(
            CONCAT_SAMPLE_STR,
            Concat::new(SOURCE_STR.iter()).to_string()
        );
    }

    #[test]
    fn it_concats_num() {
        assert_eq!(
            CONCAT_SAMPLE_NUM,
            Concat::new(SOURCE_NUM.iter()).to_string()
        );
    }
}
