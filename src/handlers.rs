use askama::Template;
use axum::{extract::Path, http::StatusCode, response::{Html, IntoResponse}};
use crate::{post, views::{PageBlogTemplate, PageCreditsTemplate, PageHomeTemplate, PageNotFoundTemplate, PagePostTemplate}};


pub async fn render_home_page() -> impl IntoResponse {
    render_template(PageHomeTemplate)
}

pub async fn render_credits_page() -> impl IntoResponse {
    render_template(PageCreditsTemplate)
}

pub async fn render_not_found_page() -> impl IntoResponse {
    render_template(PageNotFoundTemplate)
}

pub fn render_template<T: Template>(template: T) -> impl IntoResponse {
    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering template").into_response(),
    }
}

pub async fn render_blog_page() -> impl IntoResponse {
    let mut posts = match post::load_posts_from_directory("content/posts") {
        Ok(posts) => posts,
        Err(_) => Vec::new(),
    };
    posts.sort_by(|a, b| b.post_data.date.cmp(&a.post_data.date));
    render_template(PageBlogTemplate { posts })
}

pub async fn render_blog_post(Path(slug): Path<String>) -> impl IntoResponse {
    let posts = match post::load_posts_from_directory("content/posts") {
        Ok(posts) => posts,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Could not load posts").into_response(),
    };
    
    match posts.into_iter().find(|p| p.post_data.slug == slug) {
        Some(post) => render_template(PagePostTemplate { post }).into_response(),
        None => render_template(PageNotFoundTemplate).into_response(),
    }
}