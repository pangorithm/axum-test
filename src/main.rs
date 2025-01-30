mod app;
mod db;
mod routes;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a single route
    let app = app::create_app().await;

    let db = db::connection::establish_connection().await;

    let port = env::var("PORT").expect("PORT must be set");
    // run our app with hyper, listening globally on port
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("{}", format!("Listening on port {}", port));
    axum::serve(listener, app).await.unwrap();
}
