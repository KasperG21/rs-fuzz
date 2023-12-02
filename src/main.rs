use std::{error::Error, path::Path, str::FromStr};

mod fuzz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let threads = 16;
    let url = "http://127.0.0.1:8888";

    let wordlists = fuzz::load_wordlist(&Path::new("fuzz.txt"), threads).await?;

    let mut handles = vec![];
    for wordlist in wordlists {
        handles.push(tokio::spawn(async move {
            fuzz::fuzz(reqwest::Url::from_str(url).unwrap(), wordlist)
                .await
                .unwrap();
        }))
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
