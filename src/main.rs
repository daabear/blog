use axum::{response::Html, routing::get, Router};
use askama::Template;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(render_base_page))
        .route("/blog", get(render_blog_page))
        .route("/credits", get(render_credits_page))
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
struct PageBaseTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct PageBlogTemplate;

#[derive(Template)]
#[template(path = "credits.html")]
struct PageCreditsTemplate;

async fn render_base_page() -> Html<String> {
    Html(PageBaseTemplate.render().unwrap())
}

async fn render_blog_page() -> Html<String> {
    Html(PageBlogTemplate.render().unwrap())
}

async fn render_credits_page() -> Html<String> {
    Html(PageCreditsTemplate.render().unwrap())
}