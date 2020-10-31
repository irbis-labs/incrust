use std::fmt;

pub trait FilterFactory<Input: fmt::Display> {
    type Output: fmt::Display;

    fn create(&self, input: Input) -> Self::Output;
}

pub trait AbstractFilterFactory {
    fn pipe<'a>(&self, input: Box<dyn fmt::Display + 'a>) -> Box<dyn fmt::Display + 'a>;
}
