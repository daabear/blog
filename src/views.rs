use askama::Template;
use crate::post::BlogPost;

#[derive(Template)]
#[template(path = "home.html")]
pub struct PageHomeTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
pub struct PageBlogTemplate {
    pub posts: Vec<BlogPost>,
}

#[derive(Template)]
#[template(path = "credits.html")]
pub struct PageCreditsTemplate;

#[derive(Template)]
#[template(path = "post.html")]
pub struct PagePostTemplate {
    pub post: BlogPost,
}

#[derive(Template)]
#[template(path = "notfound.html")]
pub struct PageNotFoundTemplate;