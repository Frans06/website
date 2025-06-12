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
            slug: "tanstack-forms-custom-fields".to_string(),
            title: "Custom fields and layouts using Tanstack Forms".to_string(),
            excerpt: "Learn how to build a custom <Input /> component using the @tanstack/react-form library while keeping layout and logic cleanly separated. This guide shows how to wrap your form fields with a reusable InputLayout to manage titles, icons, error states, and other UI elements — all without cluttering your field logic.".to_string(),
            content:  include_str!("./posts/tanstack_forms.md").to_string(),
            category: "typescript".to_string(),
            date: "2024-12-15".to_string(),
            read_time: "10 min read".to_string(),
        },
    ];
    posts
}

pub fn get_post(slug: &str) -> Option<BlogPostData> {
    let posts = get_posts();
    posts.iter().find(|post| post.slug == slug).cloned()
}
