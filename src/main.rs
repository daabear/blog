use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode};
use askama::Template;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // build our application with a route
    let app = create_router();

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router() -> Router {  
    Router::new()
    .route("/", get(render_home_page))
    .route("/blog", get(render_blog_page))
    .route("/credits", get(render_credits_page))
    .nest_service("/assets", ServeDir::new("assets"))
}

#[derive(Template)]
#[template(path = "home.html")]
struct PageHomeTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct PageBlogTemplate;

#[derive(Template)]
#[template(path = "credits.html")]
struct PageCreditsTemplate;

async fn render_home_page() -> impl IntoResponse {
    match PageHomeTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering template").into_response(),
    }
}

async fn render_blog_page() -> impl IntoResponse {
    match PageBlogTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering template").into_response(),
    }
}

async fn render_credits_page() -> impl IntoResponse {
    match PageCreditsTemplate.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering template").into_response(),
    }
}