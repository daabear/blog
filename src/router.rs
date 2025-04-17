use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::handlers::{render_blog_page, render_blog_post, render_credits_page, render_home_page, render_not_found_page};

pub fn create_router() -> Router {      
    Router::new()
    .route("/", get(render_home_page))
    .route("/blog", get(render_blog_page))
    .route("/blog/{slug}", get(render_blog_post))
    .route("/credits", get(render_credits_page))
    .nest_service("/assets", ServeDir::new("assets"))
    .fallback(render_not_found_page)
}  