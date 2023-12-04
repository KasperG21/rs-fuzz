use std::error::Error;

use colored::{ColoredString, Colorize};

use tokio::{fs, io::AsyncReadExt};

use reqwest::{Client, StatusCode};

pub async fn load_wordlist(
    path: String,
    threads: usize,
) -> Result<(Vec<Vec<String>>, usize), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new().read(true).open(path).await?;

    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    let buf_lines: Vec<_> = buf.lines().map(|x| x.to_owned()).collect();
    let total_lines = buf_lines.len();

    let mut result = vec![];
    let length_for_each = total_lines / threads;

    if total_lines / threads < threads {
        for line in buf_lines {
            result.push(vec![line]);
        }
    } else {
        for i in 0..threads {
            result.push(
                buf_lines[(i * length_for_each)..((i * length_for_each) + length_for_each)]
                    .to_owned(),
            )
        }
    }

    Ok((result, total_lines))
}

pub async fn fuzz(url: String, wordlist: Vec<String>) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let fuzz_index = url.find("FUZZ").unwrap();

    let url_part_1 = &url[..fuzz_index];
    let url_part_2 = &url[fuzz_index + 4..];

    for path in wordlist {
        let formatted_url = format!("{}{}{}", url_part_1, path, url_part_2);

        let response_result = client.get(&formatted_url).send().await;

        let response = match response_result {
            Ok(t) => t,
            Err(e) => panic!("Error connecting. ({})", e),
        };

        println!(
            "{}{}{}",
            style_error_code(response.status()),
            " ".repeat(30 - response.status().to_string().len()),
            formatted_url
        );
    }

    client.get(url).header("Connection", "close").send().await?;

    Ok(())
}

fn style_error_code(status: StatusCode) -> ColoredString {
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
