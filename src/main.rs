use scraper::{Html, Selector};
use std::collections::HashSet;
use regex::Regex;

async fn get_words(
    url: &str
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut words = HashSet::new();

    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("body")?;
    
    let re = Regex::new(r"[A-Za-z0-9_]+")?;

    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");

        for cap in re.find_iter(&text) { 
            let word =  cap.as_str().to_lowercase();

            if (3..=32).contains(&word.len()) {
                words.insert(word);
            }
        }
    }

    Ok(words)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://netflix.com";
    let words = get_words(url).await?;

    let mut words: Vec<_> = words.into_iter().collect();
    words.sort();
    for word in words {
        println!("{word}");
    }

    Ok(())
}
