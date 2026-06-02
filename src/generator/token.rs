use std::hash::{Hash, Hasher};
use crate::generator::core::Score;

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
    pub source: String,
    pub score: Score,
    pub occurrence: u32,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum TokenKind {
    Word,
    Identifier,
    Path,
    Url,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

