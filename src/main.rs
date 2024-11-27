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
    let url = "provide your discord webhook url here";

    let post = Post {
        content: String::from("message send by a rust client in #gepruts"),
        embeds: vec![Embed {
            title: "webhook".to_string(),
            description: "this is a message from an external application".to_string(),
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

    if response.status().is_success() {
        println!("Response: {}", response.text().await?);
    } else {
        eprintln!("Failed to send POST request: {}", response.status());
    }

    Ok(())
}
