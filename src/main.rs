#![allow(unused)]
use anyhow::{Result};
use axum::body::HttpBody;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use log;
use simple_logger;
#[tokio::main]
async fn main(){
    simple_logger::init().unwrap();
    println!("Welcome to Axum Web-Server");
    // Build app with single route
    let router = Router::new().route("/", get(handler_response));
    // Listen on port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    log::info!("Server Running on Port: 3000");
    axum::serve(listener,router).await.unwrap();

}


async fn handler_response()-> impl IntoResponse{
    Html("Hello Samray")
}