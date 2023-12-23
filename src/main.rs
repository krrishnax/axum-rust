#![allow(unused)] // For beginning only.

use axum::{Router, routing::get, response::{Html, IntoResponse}, extract::Query};
use serde::Deserialize;

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
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g: `/hello?name=Krrishna`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name
        .as_deref()
        .unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// cargo watch -q -c -w src/ -x run (backend)
// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"  (client)