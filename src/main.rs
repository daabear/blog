use axum::{response::Html, routing::get, Router};
use askama::Template;
use tower_http::services::ServeDir;

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

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {}

async fn handler() -> Html<String> {
    let template = BaseTemplate {};
    Html(template.render().unwrap())
}