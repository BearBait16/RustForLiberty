// Tutorial from https://blog.logrocket.com/making-http-requests-rust-reqwest/
// additional Json parsing tutorial from https://reintech.io/blog/working-with-json-in-rust
use reqwest;
use serde_json;
use std::io;
#[tokio::main]
async fn main() {
    let mut quit = false;
    while quit == false {
        println!(
            "Welcome Soldier, to Rust for Liberty, the Helldivers 2 reporting program built in Rust!"
        );

        println!("What would you like to do soldier?");
        println!("  1. Review News from the Front lines");
        println!("  2. Review active Planets and their status in the fight for liberty");
        println!("  3. Enjoy a cup of Liber-Tea");
        println!("  4. Quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.as_str().trim() {
            "1" => display_news().await,
            "2" => display_active_planets().await,
            "3" => println!("Ahhhhhhh"),
            "4" => quit = true,
            _ => println!("Invalid option, soldier!"),
        }
    }
}

async fn display_news() {
    println!("");
    println!("Messages recieved from Super Earth are as follows: ");
    println!("");
    println!("");

    let response = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/news")
        .await
        .expect("The info has failed super earth");
    let news: Vec<serde_json::Value> = response.json().await.expect("Awh Crap");

    let mut id = 0;
    let mut message;
    for article in &news {
        id += 1;
        message = article["message"].as_str();
        println!("Article id number: {:?}", id);
        print!("Message: {:?}", message);
        println!("");
        println!("");
    }
}

async fn display_active_planets() {
    let active_planets = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/campaign")
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", active_planets);
}
