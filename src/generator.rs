use scraper::{Html, Selector};
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

// TODO:
// camelCase splitting
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
    entropy_score: f64,
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
    fn filter_token (token: &str) -> Option<Token> {
        let normalised = token.to_lowercase();

        if normalised.len() < 3 {
            return None;
        }

        if normalised.chars().all(|c| c.is_numeric()) {
            return None;
        }

        if normalised.chars().all(|c| c.is_ascii_hexdigit())
            && normalised.len() > 6 {
            return None;
        }

        let entropy = Self::calculate_entropy(&normalised);
        if !(0.9..=4.0).contains(&entropy) {
            return None;
        }

        Some(Token {
            value: normalised,
            kind: TokenKind::Word,
            source: "Body".to_string(),
            entropy_score: entropy,
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

        if let Some(token) = Self::filter_token(&raw) {
            tokens.insert(token);
        }

        for split in Self::split_token(&raw) {
            if let Some(token) = Self::filter_token(&split) {
                tokens.insert(Token {
                    value: token.value,
                    kind: TokenKind::Identifier,
                    source: "Body".to_string(),
                    entropy_score: token.entropy_score,
                });
            }
        }
    }

    fn calculate_entropy(input: &str) -> f64 {
        let mut counts = HashMap::new();

        for ch in input.chars() {
            *counts.entry(ch).or_insert(0usize) += 1;
        }

        let len = input.len() as f64;
        let mut entropy = 0.0;

        for count in counts.values() {
            let p = *count as f64 / len;
            entropy -= p * p.log2();
        }

        entropy
    }

    fn split_token(token: &str) -> Vec<String> {
        let mut results = Vec::new();

        for segment in token.split(['_', '-']) {
            if segment.is_empty() {
                continue;
            }

            let mut current = String::new();

            for (i, ch) in segment.chars().enumerate() {
                if i > 0 && ch.is_uppercase() {
                    results.push(current.to_lowercase());
                    current.clear();
                }
                
                current.push(ch);
            }

            if !current.is_empty() {
                results.push(current.to_lowercase());
            }
        }

        results
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

    pub fn entropy(&self) -> f64 {
        self.entropy_score
    }
}

