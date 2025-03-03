use reqwest;

#[tokio::main]
async fn main() {
    let result = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/status").await;
    println!("{:?}", result);
}