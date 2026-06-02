use clap::Parser;
use crate::generator::generate::extract_tokens;
use std::cmp::Ordering;

mod generator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL use to generate wordlist
    #[arg(short, long, required = true)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let body = reqwest::get(args.url).await?.text().await?;

    let tokens = extract_tokens(&body);

    for token in &tokens {
        if token.score.dictionary_match == true {
            println!("{}", token.value);
        }
    }

    Ok(())
}

