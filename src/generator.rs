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
    fn flush_token(
        current: &mut String,
        tokens: &mut HashSet<Token>,
    ) -> () {
        if current.is_empty() {
            return;
        }

        tokens.insert(Token {
            value: current.to_string(),
            kind: TokenKind::Word,
            source: "Body".to_string(),
            entropy_score: Self::calculate_entropy(current),
        });

        for split in Self::split_token(current) {
            tokens.insert(Token {
                value:split,
                kind: TokenKind::Identifier,
                source: "Body".to_string(),
                entropy_score: Self::calculate_entropy(current),
            });
        }
        
        current.clear();
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
        token
            .split(['_', '-'])
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
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

