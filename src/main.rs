use clap::Parser;
use generator::Token;
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
    let words = Token::get_tokens(&args.url).await?;

    let mut words =  words
        .into_iter()
        .collect::<Vec<_>>();

    words.sort_by(|a, b| {
        a.entropy()
            .partial_cmp(&b.entropy())
            .unwrap_or(Ordering::Equal)
    });

    for word in words {
        println!("{:?}", word);
    }

    Ok(())
}

