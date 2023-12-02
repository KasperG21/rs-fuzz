use std::{error::Error, path::Path};

use colored::{ColoredString, Colorize, CustomColor};

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
        let response_result = Client::new().get(format!("{}{}", url, path)).send().await;

        let response = match response_result {
            Ok(t) => t,
            Err(e) => panic!("Error connecting. ({})", e),
        };

        println!("{}        {}", style_error_code(response.status()), path);
    }

    Ok(())
}

fn style_error_code(status: StatusCode) -> ColoredString {
    println!("{}|{}", "test".red(), "test".bright_red());
    if status.is_informational() {
        status.to_string().blue()
    } else if status.is_success() {
        status.to_string().bright_green()
    } else if status.is_redirection() {
        status.to_string().yellow()
    } else if status.is_client_error() {
        status.to_string().red()
    } else if status.is_server_error() {
        status.to_string().bright_red()
    } else {
        status.to_string().white()
    }
}
