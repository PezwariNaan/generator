use crate::generator::{
    splitter::{CamelCaseSplitter, SnakeCaseSplitter},
    scorer::BasicScorer,
    token::{Token, TokenKind},
    core::{Splitter, Scorer, Score},
};
use std::collections::HashSet;
use std::sync::LazyLock;

// TODO:
// Add splitters

pub static DICTIONARY: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| {
        include_str!("../data/words_alpha.txt")
            .lines()
            .collect()
    });

pub fn extract_tokens(body: &str) {
    let mut filtered: Vec<String> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let scorer = BasicScorer;
    let splitter = SnakeCaseSplitter;

    for word in body.split_whitespace() {
        let normalised = normalise_and_filter(word);
        match normalised {
            Some(normalised) => filtered.push(normalised),
            None => {},
        }
    }

    for f in &filtered {
        let splits = splitter.split(f);
        for split in &splits {
            let token = build_token(split, TokenKind::Word, &scorer);
            match token {
                Some(token) => tokens.push(token),
                None => {},
            }
        }
    }

    for token in tokens {
        println!("{:?}", token);
    }
}

fn build_token(
    raw: &str,
    kind:TokenKind,
    scorer: &dyn Scorer,
    ) -> Option<Token> {

    let mut score: Score = Score::default(); 

    let normalised = normalise_and_filter(raw)?;
    score.entropy  = scorer.entropy(normalised.as_str());
    score.vowel_ratio = scorer.vowels(normalised.as_str());
    score.dictionary_match = DICTIONARY.contains(normalised.as_str());

    Some (Token {
        value: normalised,
        kind,
        source: "Body".to_string(),
        score: score,
    })
}

fn normalise_and_filter(token: &str) -> Option<String> {
    let normalised = token.to_lowercase();

    if normalised.len() < 3 || normalised.len() > 32 {
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

