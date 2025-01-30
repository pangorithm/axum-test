use axum::{routing::get, Router};

pub fn api_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, API!" }))
}
