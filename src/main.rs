use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse}, routing::get, Router};
use askama::Template;
use post::BlogPost;
use tower_http::services::ServeDir;
mod post;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = create_router();

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

fn create_router() -> Router {  
    Router::new()
    .route("/", get(render_home_page))
    .route("/blog", get(render_blog_page))
    .route("/blog/{slug}", get(render_blog_post))
    .route("/credits", get(render_credits_page))
    .nest_service("/assets", ServeDir::new("assets"))
}

#[derive(Template)]
#[template(path = "home.html")]
struct PageHomeTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct PageBlogTemplate {
    posts: Vec<BlogPost>,
}

#[derive(Template)]
#[template(path = "credits.html")]
struct PageCreditsTemplate;

#[derive(Template)]
#[template(path = "post.html")]
struct PagePostTemplate {
    post: BlogPost,
}

async fn render_home_page() -> impl IntoResponse {
    render_template(PageHomeTemplate)
}

async fn render_credits_page() -> impl IntoResponse {
    render_template(PageCreditsTemplate)
}

fn render_template<T: Template>(template: T) -> impl IntoResponse {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering template").into_response(),
    }
}

async fn render_blog_page() -> impl IntoResponse {
    let posts = match post::load_posts_from_directory("content/posts") {
        Ok(posts) => posts,
        Err(_) => Vec::new(),
    };

    render_template(PageBlogTemplate { posts })
}

async fn render_blog_post(Path(slug): Path<String>) -> impl IntoResponse {
    let posts = match post::load_posts_from_directory("content/posts") {
        Ok(posts) => posts,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Could not load posts").into_response(),
    };

    match posts.into_iter().find(|p| p.post_data.slug == slug) {
        Some(post) => render_template(PagePostTemplate { post }).into_response(),
        None => (StatusCode::NOT_FOUND, "Post not found").into_response(),
    }
}