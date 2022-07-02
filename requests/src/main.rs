use reqwest::header::HeaderMap;
use reqwest::header::*;
use reqwest::StatusCode;
use std::io;

pub mod errors;
pub mod models;

use crate::errors::{APIError, APIErrorType};
use crate::models::APIResponse;

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

async fn search(query: &str) -> io::Result<APIResponse> {
    let client = reqwest::Client::new();
    let response = client
        .get(SEARCH_URL)
        .query(&[("query", query), ("type", "track,artist")])
        .headers(build_headers())
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => response.json::<APIResponse>().await.map_err(|_| {
            APIError {
                kind: APIErrorType::Other,
                message: None,
            }
            .into()
        }),
        StatusCode::UNAUTHORIZED => Err(APIError {
            kind: APIErrorType::Unauthorized,
            message: Some("Need a new token".to_string()),
        })?,
        _ => Err(APIError {
            kind: APIErrorType::RequestError,
            message: Some("API returned unsupported response".to_string()),
        })?,
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let result = search("Little Simz").await?;
    println!("{:?}", result);

    Ok(())
}
