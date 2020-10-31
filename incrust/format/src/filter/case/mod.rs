pub mod capitalize;
pub mod lowercase;
pub mod uppercase;

pub use self::capitalize::*;
pub use self::lowercase::*;
pub use self::uppercase::*;

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt;

    use crate::{AbstractFilterFactory, FilterFactory};

    #[test]
    fn literal_chain() {
        assert_eq!("WORD", Uppercase(Lowercase("wOrD")).to_string());
        assert_eq!("word", Lowercase(Uppercase("wOrD")).to_string());
    }

    #[test]
    fn factory_chain() {
        let filter = LowercaseFactory.create("WORD");
        let filter = CapitalizeFactory.create(filter);
        assert_eq!("Word", filter.to_string())
    }

    #[test]
    fn abstract_pipeline() {
        let filters: Vec<Box<dyn AbstractFilterFactory>> =
            vec![Box::new(LowercaseFactory), Box::new(CapitalizeFactory)];
        let mut filter: Box<dyn fmt::Display> = Box::new("WORD".to_string());
        for f in &filters {
            filter = f.pipe(filter);
        }
        assert_eq!("Word", filter.to_string())
    }
}