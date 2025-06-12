use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

#[derive(Clone)]
struct BlogPostData {
    slug: String,
    title: String,
    excerpt: String,
    category: String,
    date: String,
    read_time: String,
}
#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").clone().unwrap_or_default());

    // Mock blog post content - in real app, this would fetch from a CMS or database
    let get_post_content = move |slug: String| -> Option<BlogPostData> {
        match slug.as_str() {
            "rust-performance-optimization" => Some(BlogPostData {
                slug: slug.clone(),
                title: "Rust Performance Optimization in Production".to_string(),
                excerpt:
                    "Deep dive into optimizing Rust applications for high-performance web services"
                        .to_string(),
                category: "Rust".to_string(),
                date: "December 15, 2024".to_string(),
                read_time: "8 min read".to_string(),
            }),
            _ => None,
        }
    };

    view! {
        <div class="pt-32 pb-20 px-4">
            <div class="max-w-4xl mx-auto">
                {move || {
                    if let Some(post) = get_post_content(slug()) {
                        view! {
                            <article>
                                <header class="mb-12">
                                    <div class="flex items-center gap-4 mb-6">
                                        <A href="/blog">
                                <p class="flex items-center text-cyan-400 hover:text-purple-400 transition-colors">
                                            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                            </svg>
                                            "Back to Blog"
                                </p>
                                        </A>
                                    </div>

                                    <div class="flex items-center gap-4 mb-6">
                                        <span class="px-3 py-1 bg-gradient-to-r from-purple-500/20 to-cyan-500/20 rounded-full text-sm border border-purple-500/30 text-cyan-400">
                                            {post.category}
                                        </span>
                                        <span class="text-gray-400">{post.date}</span>
                                        <span class="text-gray-400">-</span>
                                        <span class="text-gray-400">{post.read_time}</span>
                                    </div>

                                    <h1 class="orbitron text-4xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500 mb-6">
                                        {post.title}
                                    </h1>
                                </header>

                                <div class="prose prose-invert prose-lg max-w-none">
                                    <BlogContent/>
                                </div>
                            </article>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center">
                                <h1 class="orbitron text-4xl font-bold text-red-400 mb-4">"404 - Post Not Found"</h1>
                                <p class="text-gray-300 mb-8">"The blog post you're looking for doesn't exist."</p>
                                <A href="/blog" >
                                <p class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-purple-600 to-cyan-600 rounded-full hover:from-purple-500 hover:to-cyan-500 transition-all duration-300 font-semibold">
                                    "Back to Blog"
                                </p>
                                </A>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

#[component]
fn BlogContent() -> impl IntoView {
    view! {
        <div class="card-hover rounded-xl p-8 space-y-6">
            <p class="text-lg text-gray-300 leading-relaxed">
                "Performance optimization in Rust requires understanding both the language's unique features and the underlying system architecture. In this post, we'll explore advanced techniques for building high-performance web services."
            </p>

            <h2 class="orbitron text-2xl font-bold text-cyan-400 mt-8 mb-4">"Memory Management Strategies"</h2>
            <p class="text-gray-300 leading-relaxed">
                "Rust's ownership system provides zero-cost abstractions, but understanding when and how to use different allocation strategies can significantly impact performance. We'll cover stack vs heap allocation, custom allocators, and memory pooling techniques."
            </p>

            <h2 class="orbitron text-2xl font-bold text-cyan-400 mt-8 mb-4">"Async Performance Patterns"</h2>
            <p class="text-gray-300 leading-relaxed">
                "Modern web services rely heavily on asynchronous programming. We'll examine Tokio runtime tuning, efficient task spawning, and avoiding common async pitfalls that can degrade performance."
            </p>

            <div class="bg-gray-800 rounded-lg p-6 border border-gray-700">
                <h3 class="orbitron text-lg font-bold text-purple-400 mb-3">"Key Takeaways"</h3>
                <ul class="list-disc list-inside text-gray-300 space-y-2">
                    <li>"Profile before optimizing - use tools like perf and flamegraph"</li>
                    <li>"Understand your allocation patterns and minimize heap usage"</li>
                    <li>"Choose the right async runtime configuration for your workload"</li>
                    <li>"Leverage Rust's type system for compile-time optimizations"</li>
                </ul>
            </div>
        </div>
    }
}
