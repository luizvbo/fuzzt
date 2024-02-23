mod simple_processors;
pub use simple_processors::{LowerAlphaNumStringProcessor, NullStringProcessor};

pub trait StringProcessor {
    fn process(&self, s: &str) -> String;
}
