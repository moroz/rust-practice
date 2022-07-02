use reqwest::{self, StatusCode};
use std::io;

const SEARCH_URL: &str = "https://api.spotify.com/v1/search";

async fn search() -> String {
    return reqwest::get(SEARCH_URL)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let result = search().await;
    println!("{}", result);

    Ok(())
}
