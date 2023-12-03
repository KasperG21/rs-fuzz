use std::{error::Error, path::Path, str::FromStr};

use tokio::time::Instant;

mod fuzz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let threads = 16;
    let url = "http://127.0.0.1:8888";

    let (wordlists, file_len) = fuzz::load_wordlist(&Path::new("fuzz.txt"), threads).await?;

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

    println!(
        "
--------------------FINISHED--------------------
Took {:?}.
Fuzzed {} URL's.
        ",
        start.elapsed(),
        file_len
    );

    Ok(())
}
