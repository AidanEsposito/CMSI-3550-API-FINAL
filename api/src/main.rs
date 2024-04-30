use axum::{
    routing::{get},
    Router,
    response::{ Json},
};
use serde::Deserialize;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(get_joke));

    // let addr = "0.0.0.0:3000".parse()?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn get_joke() -> Json<String> {
    let client = Client::new();
    let response = client
        .get("https://api.chucknorris.io/jokes/random")
        .send()
        .await
        .expect("Failed to fetch Chuck Norris joke");

    if response.status().is_success() {
        let body = response.text().await.expect("Failed to read response body");
        let joke: ChuckNorrisJoke = serde_json::from_str(&body)
            .expect("Failed to deserialize Chuck Norris joke");
        Json(joke.value)
    } else {
        Json("Failed to fetch Chuck Norris joke".to_string())
    }
}

#[derive(Deserialize)]
struct ChuckNorrisJoke {
    value: String,
}
