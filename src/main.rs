use std::{error::Error, path::Path, str::FromStr};

mod fuzz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_wordlist = fuzz::load_wordlist(&Path::new("fuzz.txt")).await?;
    fuzz::fuzz(
        reqwest::Url::from_str("http://127.0.0.1:8888")?,
        String::from_utf8_lossy(raw_wordlist.as_slice()).to_string(),
    )
    .await?;

    Ok(())
}
