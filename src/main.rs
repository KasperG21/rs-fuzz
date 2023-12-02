use std::{error::Error, path::Path, str::FromStr};

mod fuzz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let threads = 16;

    let wordlists = fuzz::load_wordlist(&Path::new("fuzz.txt"), threads).await?;

    Ok(())
}

async fn handle(wordlist: String) {
    fuzz::fuzz(
        reqwest::Url::from_str("http://127.0.0.1:8888").unwrap(),
        wordlist,
    )
    .await
    .unwrap();
}
