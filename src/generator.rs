use scraper::{Html, Selector};
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

// TODO:
// Refactor Token into seperate classes
// Attribute extraction
// JS parsing

#[derive(Debug, Hash, Eq, PartialEq)]
enum TokenKind {
    Word,
    Identifier,
    Path,
    Url,
}

#[derive(Debug)]
pub struct Token {
    value: String,
    kind: TokenKind,
    source: String,
    score: Score,
}

#[derive(Debug)]
pub struct Score {
    entropy: f64,
    vowel_ratio: f64,
    dictionary_match: bool,
    digit_ratio: f64,
}

struct SnakeCaseSplitter;
struct CamelCaseSplitter;

trait Splitter {
    fn split(&self, input: &str) -> Vec<String>;
}

impl Splitter for SnakeCaseSplitter {
    fn split(&self, input: &str) -> Vec<String> {
        input
            .split(['_','-'])
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
}

impl Splitter for CamelCaseSplitter {
    fn split(&self, input: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut current = String::new();

        for (i, ch) in input.chars().enumerate() {
            if i > 0 && ch.is_uppercase() && !current.is_empty() {
                results.push(current.to_lowercase());
                current.clear();
            }

            current.push(ch);
        }

        if !current.is_empty() {
            results.push(current.to_lowercase());
        }

        results
    }
}

impl Score {
    fn from_token(token: &str) -> Self {
        let entropy = Self::entropy(token);
        let vowel_ratio = Self::vowels(token);

        Self {
            entropy,
            vowel_ratio,
            dictionary_match: false,
            digit_ratio: 0.0,
        }
    }
    
    fn score(&self) -> f64 {
        self.entropy +
        self.vowel_ratio +
        self.digit_ratio
    }

    fn entropy(token: &str) -> f64 {
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

    fn vowels(token: &str) -> f64 {
        let vowel_count = token
            .chars()
            .filter(|c| "aeiou".contains(*c))
            .count();

        vowel_count as f64 / token.len() as f64
    }
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

impl Token {
    pub fn score(&self) -> f64 {
        self.score.score()
    }

    fn normalise(token: &str) -> Option<String> {
        let normalised = token.to_lowercase();

        if normalised.len() < 3 || normalised.len() > 32 {
            return None;
        }

        if normalised.chars().all(|c| c.is_numeric()) {
            return None;
        }

        if normalised.chars().all(|c| c.is_ascii_hexdigit())
            && normalised.len() > 4 {
            return None;
        }

        Some(normalised)
    }

    fn build_token (token: &str, kind: TokenKind) -> Option<Token> {
        let normalised = Self::normalise(token)?;

        let score = Score::from_token(&normalised);

        Some(Token {
            value: normalised,
            kind: TokenKind::Word,
            source: "Body".to_string(),
            score: score,
        })
    }

    fn flush_token(
        current: &mut String,
        tokens: &mut HashSet<Token>,
    ) -> () {
        if current.is_empty() {
            return;
        }

        let raw = current.clone();
        current.clear();

        if let Some(token) = Self::build_token(&raw, TokenKind::Word) {
            tokens.insert(token);
        }

        let splitters: Vec<Box<dyn Splitter>> = vec![
            Box::new(SnakeCaseSplitter),
            Box::new(CamelCaseSplitter),
        ];

        for splitter in &splitters {
            for split in splitter.split(&raw) {
                if let Some(mut token) = Self::build_token(&split, TokenKind:: Identifier) {
                    token.kind = TokenKind::Identifier;
                    tokens.insert(token);
                }
            }
        }
    }

    fn extract_tokens(text: &str) -> HashSet<Token> {
        let mut tokens = HashSet::new();
        let mut current = String::new();

        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                current.push(ch);
            } else {
                Self::flush_token(
                    &mut current, 
                    &mut tokens,
                );
            }
        }

        // Flush final token
        Self::flush_token(
            &mut current,
            &mut tokens,
        );
        tokens
    }

    pub async fn get_tokens(
        url: &str
    ) -> Result<HashSet<Token>, Box<dyn std::error::Error>> {
        let mut tokens = HashSet::new();
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse("body")?;
        
        for element in document.select(&selector) {
            let text = element.text().collect::<Vec<_>>().join(" ");

            tokens.extend(Self::extract_tokens(&text));
        }

        Ok(tokens)
    }
}


