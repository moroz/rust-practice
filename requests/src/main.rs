use reqwest::{self, StatusCode};
use std::io;

const SEARCH_URL: &str = "https://api.spotify.com/v1/search";

#[tokio::main]
async fn main() -> io::Result<()> {
    let response = reqwest::get(SEARCH_URL).await.unwrap();
    match response.status() {
        StatusCode::OK => {
            println!("Success! {:?}", response);
        }
        StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened!");
        }
    }

    Ok(())
}
