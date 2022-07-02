use reqwest;

#[tokio::main]
async fn main() {
    let result = reqwest::get("https://moroz.dev").await;
    println!("{:?}", result);
}
