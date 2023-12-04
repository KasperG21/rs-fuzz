use std::error::Error;

use tokio::time::Instant;

mod args;
mod fuzz;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let args = match args::args() {
        Ok(t) => t,
        Err(_) => return Ok(()),
    };

    let url = args.url();
    let wordlist_path = args.wordlist();
    let threads = args.threads();

    let (wordlists, file_len) = fuzz::load_wordlist(wordlist_path, threads).await?;

    let mut handles = vec![];
    for wordlist in wordlists {
        let url = url.clone();
        handles.push(tokio::spawn(async move {
            fuzz::fuzz(url, wordlist).await.unwrap();
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
