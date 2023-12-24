#![allow(unused)] // For beginning only.

use axum::{
    Router, 
    routing::{get, get_service}, 
    response::{Html, IntoResponse, Response}, 
    extract::{Query, Path}, middleware
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
use crate::model::ModelController;
pub use self::error::{Error, Result};

mod web;
mod model;
mod ctx;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController.
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region:   --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
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