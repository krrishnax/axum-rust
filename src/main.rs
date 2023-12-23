#![allow(unused)] // For beginning only.

use axum::{Router, routing::get, response::{Html, IntoResponse}};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region:   --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on {:?}\n", listener);

    axum::serve(listener, routes_hello)
        .await
        .unwrap()
}

// region:   --- Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
}

// cargo watch -q -c -w src/ -x run (backend)
// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"  (client)