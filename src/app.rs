use crate::routes::api::api_routes;
use axum::{routing::get, Router};
// use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};

pub async fn create_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api_routes())
    // .layer(NormalizePathLayer::trim_trailing_slash())
    // .layer(CorsLayer::permissive()) // CORS 설정
}
