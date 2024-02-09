pub trait StringProcessor {
    fn process<'a>(&self, s:&'a str) -> String;
}

pub struct LowerAlphaNumStringProcessor;
pub struct NullStringProcessor;


impl StringProcessor for LowerAlphaNumStringProcessor {
    fn process<'a>(&self, input: &'a str) -> String {
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
    fn process<'a>(&self, input: &'a str) -> String {
        input.to_owned()
    }
}
