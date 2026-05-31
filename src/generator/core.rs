#[derive(Debug, Default)]
pub struct Score {
    pub entropy: f64,
    pub vowel_ratio: f64,
    pub dictionary_match: bool,
    pub digit_ratio: f64,
}

pub trait Splitter {
    fn split(&self, input: &str) -> Vec<String>;
}

pub trait Scorer {
    fn entropy(&self, token: &str) -> f64;
    fn vowels(&self, token: &str) -> f64;
}

