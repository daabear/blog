use std::fs;
use chrono::NaiveDate;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;    

#[derive(Debug, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub slug: String,
    pub date: NaiveDate,
    pub summary: String,
}
pub struct BlogPost {
    pub post_data: PostMetadata,
    pub content: String,
}

pub fn load_posts_from_directory(directory_path: &str) -> Result<Vec<BlogPost>, std::io::Error> {
    let mut blog_posts = Vec::new();
    let entries = fs::read_dir(directory_path).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();

        let file_content = fs::read_to_string(&path).unwrap();

        if let Some((frontmatter, body)) = split_frontmatter(&file_content) {
            let meta: PostMetadata = serde_yaml::from_str(frontmatter).unwrap();

            let html = markdown_to_html(body);

            blog_posts.push(BlogPost {
                post_data: meta,
                content: html,
            });
        }
    }

    Ok(blog_posts)
}

fn split_frontmatter(s: &str) -> Option<(&str, &str)> {
    let mut segments  = s.splitn(3, "---");  
    segments.next()?;  

    let frontmatter_text = segments.next()?.trim();
    let markdown_body_text = segments.next()?.trim();
    
    Some((frontmatter_text, markdown_body_text))
}

fn markdown_to_html(markdown_text: &str) -> String {
    let parser = Parser::new_ext(markdown_text, Options::all());
    let mut html_output = String::new();  
    html::push_html(&mut html_output, parser);  
    
    html_output  
}