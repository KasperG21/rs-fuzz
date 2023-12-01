use std::{error::Error, path::Path};

use tokio::{fs, io::AsyncReadExt};

use reqwest::{Client, StatusCode, Url};

pub async fn load_wordlist(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut file = fs::OpenOptions::new().read(true).open(path).await?;

    let mut buf = vec![];
    file.read_to_end(&mut buf).await?;

    Ok(buf)
}

pub async fn fuzz(url: Url, wordlist: String) -> Result<(), Box<dyn Error>> {
    for path in wordlist.lines() {
        let response = Client::new()
            .get(format!("{}/{}", url, path))
            .send()
            .await?;

        println!("{}    {}", path, response.status());
    }

    Ok(())
}
