use leptos::prelude::*;
use leptos_router::components::A;

use crate::posts::{get_posts, BlogPostData};

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <div class="pt-32 pb-20 px-4">
            <div class="max-w-6xl mx-auto">
                <div class="text-center mb-16">
                    <h1 class="courier text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500 mb-4">
                        "ENGINEERING LOG"
                    </h1>
                    <p class="text-xl text-gray-300 max-w-3xl mx-auto">
                        "Insights, discoveries, and technical adventures from the frontier of software engineering"
                    </p>
                </div>

                <BlogGrid/>
            </div>
        </div>
    }
}

// Island component for interactive blog functionality
#[island]
fn BlogGrid() -> impl IntoView {
    let (search_term, set_search_term) = signal(String::new());
    let (selected_category, set_selected_category) = signal("All".to_string());

    let blog_posts = get_posts();

    let categories = vec!["All", "Rust", "Cloud", "Machine Learning", "TypeScript"];

    let filtered_posts = move || {
        blog_posts
            .iter()
            .filter(|post| {
                let matches_search = search_term.get().is_empty()
                    || post
                        .title
                        .to_lowercase()
                        .contains(&search_term.get().to_lowercase())
                    || post
                        .excerpt
                        .to_lowercase()
                        .contains(&search_term.get().to_lowercase());

                let matches_category =
                    selected_category.get() == "All" || post.category == selected_category.get();

                matches_search && matches_category
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    view! {
        <div class="space-y-8">
            // Search and Filter Controls
            <div class="flex flex-col md:flex-row gap-4 justify-between items-center">
                <div class="relative flex-1 max-w-md">
                    <input
                        type="text"
                        placeholder="Search posts..."
                        class="w-full px-4 py-3 bg-gray-800 border border-gray-600 rounded-lg focus:ring-2 focus:ring-cyan-400 focus:border-transparent text-white placeholder-gray-400"
                        on:input=move |ev| set_search_term.set(event_target_value(&ev))
                        prop:value=search_term
                    />
                    <svg class="absolute right-3 top-3 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                    </svg>
                </div>

                <div class="flex gap-2 flex-wrap">
                    {categories.into_iter().map(|category| {
                        view! {
                            <button
                                class=move || format!(
                                    "px-4 py-2 rounded-full text-sm font-medium transition-all {}",
                                    if selected_category.get() == category {
                                        "bg-gradient-to-r from-cyan-500 to-purple-500 text-white"
                                    } else {
                                        "bg-gray-800 text-gray-300 hover:bg-gray-700"
                                    }
                                )
                                on:click={
                                    move |_| set_selected_category.set(category.to_string())
                                }
                            >
                                {category}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Blog Posts Grid
            <div class="grid md:grid-cols-2 gap-8">
                {move || filtered_posts().into_iter().map(|post| view! {
                    <BlogPostCard post=post.clone()/>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn BlogPostCard(post: BlogPostData) -> impl IntoView {
    view! {
        <A href=format!("/blog/{}", post.slug)>
        <div  class="block group">
            <div class="card-hover rounded-xl p-6 h-full">
                <div class="flex items-center justify-between mb-4">
                    <span class="px-3 py-1 bg-gradient-to-r from-purple-500/20 to-cyan-500/20 rounded-full text-sm border border-purple-500/30 text-cyan-400">
                        {post.category}
                    </span>
                    <span class="text-gray-500 text-sm">{post.date}</span>
                </div>

                <h3 class="courier text-xl font-bold text-white mb-3 group-hover:text-cyan-400 transition-colors">
                    {post.title}
                </h3>

                <p class="text-gray-300 leading-relaxed mb-4">
                    {post.excerpt}
                </p>

                <div class="flex items-center justify-between">
                    <span class="text-gray-400 text-sm">{post.read_time}</span>
                    <div class="flex items-center text-cyan-400 group-hover:text-purple-400 transition-colors">
                        "Read more"
                        <svg class="w-4 h-4 ml-2 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                        </svg>
                    </div>
                </div>
            </div>
            </div>
        </A>
    }
}
