use std::fmt;

pub struct Join<Sep, Iter>(pub Sep, pub Iter);

impl<Sep, Iter, Item> fmt::Display for Join<Sep, Iter>
where
    Sep: fmt::Display,
    Iter: Iterator<Item = Item> + Clone,
    Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.1.clone();
        if let Some(item) = it.next() {
            item.fmt(f)?;
        }
        for item in it {
            self.0.fmt(f)?;
            item.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Capitalize, WrapTag};
    use std::fmt::Display;

    #[test]
    fn join_str() {
        let source = "Sirius|Canopus|Rigil Kentaurus & Toliman|Arcturus";
        let sample = "Sirius, Canopus, Rigil Kentaurus & Toliman, Arcturus";
        assert_eq!(sample, Join(", ", source.split('|')).to_string());
    }

    #[test]
    fn join_num() {
        let source = [1, 2, 3];
        let sample = "1; 2; 3";
        assert_eq!(sample, Join("; ", source.iter()).to_string());
    }

    #[test]
    fn join_dyn() {
        let source: &[&dyn Display] = &[
            &1,
            &"banana",
            &2,
            &Capitalize("orange"),
            &3,
            &WrapTag("b", Join("-", "apple".split("").filter(|c| !c.is_empty()))),
        ];
        let sample = "1; banana; 2; Orange; 3; <b>a-p-p-l-e</b>";
        assert_eq!(sample, Join("; ", source.iter()).to_string());
    }
}
