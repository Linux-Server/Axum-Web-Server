#![allow(unused)]
use anyhow::{Result};
#[tokio::main]
async fn main()-> Result<()> {
    println!("The axum web-server");

    let body = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await?
        .text()
        .await?;

    // println!("body = {}", body);
    Ok(())
}