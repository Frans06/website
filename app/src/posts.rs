#[derive(Clone)]
pub struct BlogPostData {
    pub slug: String,
    pub title: String,
    pub excerpt: String,
    pub category: String,
    pub date: String,
    pub read_time: String,
    pub content: String,
}

pub fn get_posts() -> Vec<BlogPostData> {
    let posts: Vec<BlogPostData> = vec![
        BlogPostData {
            slug: "rust-performance-optimization".to_string(),
            title: "Rust Performance Optimization in Production".to_string(),
            excerpt: "Deep dive into optimizing Rust applications for high-performance web services, covering memory management, async patterns, and profiling techniques.".to_string(),
            content:  "# Comming soon".to_string(),
            category: "Rust".to_string(),
            date: "2024-12-15".to_string(),
            read_time: "8 min read".to_string(),
        },
    ];
    posts
}

pub fn get_post(slug: &str) -> Option<BlogPostData> {
    let posts = get_posts();
    posts.iter().find(|post| post.slug == slug).cloned()
}
