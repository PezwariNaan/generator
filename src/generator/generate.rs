use crate::generator::{
    splitter::{CamelCaseSplitter, SnakeCaseSplitter},
    scorer::BasicScorer,
    token::{Token, TokenKind},
    core::{Splitter, Scorer, Score},
};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

// TODO:
// Add splitters

pub static DICTIONARY: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        include_str!("../data/words_alpha.txt")
            .lines()
            .collect()
    });

pub static EXCLUDE: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        include_str!("../data/exclude_keywords.txt")
            .lines()
            .map(|s| s.trim())
            .collect()
    });

pub fn extract_tokens(body: &str) -> HashSet<Token> {
    let mut filtered: HashMap<String, u32> = HashMap::new();
    let mut tokens: HashSet<Token> = HashSet::new();
    let scorer = BasicScorer;
    let splitter = SnakeCaseSplitter;

    for word in body.split_whitespace() {
        if let Some(normalised) = normalise_and_filter(word) {
            *filtered.entry(normalised.clone()).or_insert(0) += 1;
        } 
    }

    for (word, count) in &filtered {
        let splits = splitter.split(word);
        for split in &splits {
            let occurrence = filtered
                .get(split)
                .copied()
                .unwrap_or(0);
            let token = build_token(split.as_str(), TokenKind::Word, &scorer, occurrence);
            match token {
                Some(token) => _ = tokens.insert(token),
                None => {},
            }
        }
    }

    tokens
}

fn build_token(
    raw: &str,
    kind:TokenKind,
    scorer: &dyn Scorer,
    occurrence: u32,
    ) -> Option<Token> {

    let mut score: Score = Score::default(); 

    let normalised = normalise_and_filter(raw)?;
    score.entropy  = scorer.entropy(normalised.as_str());
    score.vowel_ratio = scorer.vowels(normalised.as_str());
    score.digit_ratio = scorer.digits(normalised.as_str());
    score.dictionary_match = DICTIONARY.contains(normalised.as_str());

    Some (Token {
        value: normalised,
        kind,
        source: "Body".to_string(),
        score: score,
        occurrence,
    })
}

fn normalise_and_filter(token: &str) -> Option<String> {
    let normalised = token.to_lowercase();

    if normalised.len() < 3 || normalised.len() > 32 {
        return None;
    }

    if EXCLUDE.contains(normalised.as_str()) {
        return None;
    }

    if normalised.chars().all(|c| c.is_numeric()) {
        return None;
    }

    if normalised.chars().all(|c| !c.is_alphabetic()) {
        return None
    }

    if normalised.chars().all(|c| c.is_ascii_hexdigit())
        && normalised.len() > 4 {
        return None;
    }

    Some(normalised)
}

