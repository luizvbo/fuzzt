pub trait StringProcessor {
    fn process(&self, s: &str) -> String;
}

pub struct LowerAlphaNumStringProcessor;
pub struct NullStringProcessor;

impl StringProcessor for LowerAlphaNumStringProcessor {
    fn process(&self, input: &str) -> String {
        let processed: String = input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .trim()
            .to_lowercase();
        processed
    }
}

impl StringProcessor for NullStringProcessor {
    fn process(&self, input: &str) -> String {
        input.to_owned()
    }
}
