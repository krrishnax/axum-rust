#![allow(unused)] // For beginning only.

use axum::{Router, routing::{get, get_service}, response::{Html, IntoResponse}, extract::{Query, Path}};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // region:   --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on {:?}\n", listener);

    axum::serve(listener, routes_all)
        .await
        .unwrap()
}

// e.g: `http://localhost:8080/src/main.rs`
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:   --- Router Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

// region:   --- Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

// e.g: `http://localhost:8080/hello?name=Krrishna`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name
        .as_deref()
        .unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g: `http://localhost:8080/hello2/Palavi`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}