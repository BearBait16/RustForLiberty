// Tutorial from https://blog.logrocket.com/making-http-requests-rust-reqwest/
// additional Json parsing tutorial from https://reintech.io/blog/working-with-json-in-rust
use reqwest;
use std::io;
#[tokio::main]
async fn main() {
    let option1 = String::from("1");
    let option2 = String::from("2");
    let option3 = String::from("3");
    let option4 = String::from("4");

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
    let news: Result<String, reqwest::Error> =
        reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/news")
            .await
            .unwrap()
            .text()
            .await;
    println!("{:#?}", news);
}

async fn display_active_planets() {
    let active_planets = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/campaign")
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", active_planets);
}
