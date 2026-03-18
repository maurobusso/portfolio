use std::fs;
use chrono::NaiveDate;
// We reach into our models file to get the BlogPost definition
use crate::models::BlogPost;

pub fn load_blog_posts() -> Vec<BlogPost> {
    let mut posts = Vec::new();
    let blog_dir = "content/blog";

    // Note: You need walkdir in scope or use the full path
    for entry in walkdir::WalkDir::new(blog_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        let raw_content = fs::read_to_string(path).unwrap_or_default();

        let mut title = String::new();
        let mut date_str = String::new();
        let mut category = String::new();
        let mut status = String::from("complete");
        let mut content_lines: Vec<&str> = Vec::new(); // Added type here to fix E0282
        let mut found_content = false;

        for line in raw_content.lines() {
            if line.starts_with("# ") {
                title = line.trim_start_matches("# ").to_string();
            } else if line.contains("*Published:") {
                date_str = line.split(':').nth(1).unwrap_or("").trim().trim_matches('*').to_string();
            } else if line.contains("*Category:") {
                category = line.split(':').nth(1).unwrap_or("").trim().trim_matches('*').to_string();
            } else if line.contains("*Status:") {
                status = line.split(':').nth(1).unwrap_or("").trim().trim_matches('*').to_string();
            } else if !line.trim().is_empty() && !line.starts_with('*') {
                found_content = true;
            }

            if found_content {
                content_lines.push(line);
            }
        }

        let markdown_input = content_lines.join("\n");
        let options = pulldown_cmark::Options::empty();
        let parser = pulldown_cmark::Parser::new_ext(&markdown_input, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        let date = NaiveDate::parse_from_str(&date_str, "%B %e, %Y")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());

        posts.push(BlogPost {
            title,
            slug: path.file_stem().unwrap().to_str().unwrap().to_string(),
            content: html_output,
            date,
            category,
            excerpt: content_lines.first().cloned().unwrap_or_default().to_string(),
            status,
        });
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}