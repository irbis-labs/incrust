use crate::prelude::*;

pub struct Join<T, S> {
    pub iter: T,
    pub sep: S,
}

impl<T, S> Join<T, S> {
    pub fn new(iter: T, sep: S) -> Self {
        Join { iter, sep }
    }
}

impl<T, S> fmt::Display for Join<T, S>
where
    T: Iterator + Clone,
    T::Item: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.iter.clone();

        if let Some(first) = it.next() {
            first.fmt(f)?;
        }

        for item in it {
            self.sep.fmt(f)?;
            item.fmt(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::JOIN_SAMPLE_NUM;
    use super::super::tests::JOIN_SAMPLE_STR;
    use super::super::tests::SOURCE_NUM;
    use super::super::tests::SOURCE_STR;
    use super::*;

    #[test]
    fn it_joins_str() {
        assert_eq!(
            JOIN_SAMPLE_STR,
            Join::new(SOURCE_STR.iter(), ", ").to_string()
        );
    }

    #[test]
    fn it_joins_num() {
        assert_eq!(
            JOIN_SAMPLE_NUM,
            Join::new(SOURCE_NUM.iter(), ", ").to_string()
        );
    }
}
