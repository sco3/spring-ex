use serde::{Serialize};
use reqwest::Client;
use std::env;

#[derive(Serialize)]
struct Todo {
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <todo title>", args[0]);
        return Ok(());
    }

    let title = &args[1];
    let todo = Todo {
        title: title.to_string(),
        completed: false,
    };

    let client = Client::new();
    let res = client.post("http://localhost:8080/todos")
        .json(&todo)
        .send()
        .await?;

    let body = res.text().await?;
    println!("{}", body);

    Ok(())
}