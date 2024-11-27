use reqwest::Client;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct Embed {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct Post {
    content: String,
    embeds: Vec<Embed>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "PROVIDE_YOUR_WEBHOOK_URL_HERE";

    let post = Post {
        content: String::from("message send by a rust client in #gepruts"),
        embeds: vec![Embed {
            title: String::from("webhook"),
            description: String::from("Check out the code @ https://github.com/ReneKrewinkel/discord-api")
        }],
    };

    let client = Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&post)
        .send()
        .await?;

    match response.status().is_success() {
        true => println!("Response: {}", response.status()),
        false => eprintln!("Failed to send POST request: {}", response.status()),
    }

    Ok(())
}
