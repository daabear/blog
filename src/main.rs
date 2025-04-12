use axum::{response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;
use tokio::fs;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .nest_service("/templates", ServeDir::new("templates"))
        .nest_service("/assets", ServeDir::new("assets"));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    match fs::read_to_string("templates/index.html").await {
        Ok(index_file) => Html(index_file),
        Err(_) => {
            Html("<html><body><h1>Failed to load the page!</h1></body></html>".to_string())
        }
    }
}