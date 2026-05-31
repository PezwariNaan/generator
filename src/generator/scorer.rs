use std::collections::HashMap;
use crate::generator::core::Score;

pub struct BasicScorer;

impl crate::generator::core::Scorer for BasicScorer {
    fn entropy(&self, token: &str) -> f64 {
        let mut counts = HashMap::new();

        for ch in token.chars() {
            *counts.entry(ch).or_insert(0usize) += 1;
        }

        let len = token.len() as f64;
        let mut entropy = 0.0;

        for count in counts.values() {
            let p = *count as f64 / len;
            entropy -= p * p.log2();
        }

        entropy
    }

    fn vowels(&self, token: &str) -> f64 {
        let vowel_count = token
            .chars()
            .filter(|c| "aeiou".contains(*c))
            .count();

        vowel_count as f64 / token.len() as f64
    }
}

