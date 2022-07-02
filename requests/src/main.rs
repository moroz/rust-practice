use reqwest::header::HeaderMap;
use reqwest::header::*;
use reqwest::{self};
use std::io;

fn get_api_token() -> String {
    std::env::var("SPOTIFY_OAUTH_TOKEN")
        .expect("Environment variable SPOTIFY_OAUTH_TOKEN is not set!")
}

fn build_headers() -> HeaderMap {
    let token = get_api_token();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers
}

const SEARCH_URL: &str = "https://api.spotify.com/v1/search";

async fn search(query: &str) -> String {
    let client = reqwest::Client::new();
    return client
        .get(SEARCH_URL)
        .query(&[("query", query), ("type", "track,artist")])
        .headers(build_headers())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let result = search("Little Simz").await;
    println!("{}", result);

    Ok(())
}
