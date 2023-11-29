use reqwest;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://codefever.be/")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", response);

    Ok(())
}
