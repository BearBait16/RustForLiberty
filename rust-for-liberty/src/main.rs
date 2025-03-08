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
            "3" => cup_of_liber_tea(),
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
    println!("");
    println!(
        "Here are the active planets where liberty is being attacked! Choose these planets hero!"
    );
    let response = reqwest::get("https://helldiverstrainingmanual.com/api/v1/war/campaign")
        .await
        .expect("This info has also failed super earth");

    let active_planets: Vec<serde_json::Value> = response.json().await.expect("Awh Crap");
    let mut planet_name;
    let mut reigning_faction;
    println!("");
    println!("");
    for planet in &active_planets {
        planet_name = planet["name"].as_str();
        reigning_faction = planet["faction"].as_str();
        println!("");
        println!("");
        println!("Active Planet Name: {:?}", planet_name);
        println!("Reigning Faction: {:?}", reigning_faction);
    }
}
fn cup_of_liber_tea() {
    println!("Uhhhh. I couldn't igure out how to run an mp3 in the terminal.");
    println!("So, this is the closest thing I can get to singing it.");
    println!("BAH BAH BAH BAH");
    println!("BAH... BAH");
    println!("BAH BAH BAH BAH");
    println!("BAH.. BAH");
    println!("");
    println!("");
    println!("Thank you");
    println!("");
    println!("");
}
