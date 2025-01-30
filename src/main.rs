mod app;
mod db;
mod routes;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a single route
    let app = app::create_app().await;

    let db = db::connection::establish_connection().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
