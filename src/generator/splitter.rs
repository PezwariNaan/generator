use crate::generator::core::Splitter;

struct SnakeCaseSplitter;
struct CamelCaseSplitter;

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

