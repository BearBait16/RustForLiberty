// Tutorial from https://blog.logrocket.com/making-http-requests-rust-reqwest/

use reqwest;

#[tokio::main]
async fn main() {
    let news = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/news").await.unwrap().text().await;
    println!("{:?}", news);

    let active_planets = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/campaign").await.unwrap().text().await;
    println!("{:?}", activePlanets);
}