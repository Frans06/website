use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::posts::{get_post, BlogPostData};

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").clone().unwrap_or_default());

    // Mock blog post content - in real app, this would fetch from a CMS or database

    view! {
        <div class="pt-32 pb-20 px-4">
            <div class="max-w-4xl mx-auto">
                {move || {
                    if let Some(post) = get_post(&slug()) {
                        let iternal_post = post.clone();
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

                                    <h1 class="courier text-4xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500 mb-6">
                                        {post.title}
                                    </h1>
                                </header>

                                <div class="prose prose-invert prose-lg max-w-none">
                                    <BlogContent post=iternal_post/>
                                </div>
                            </article>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center">
                                <h1 class="courier text-4xl font-bold text-red-400 mb-4">"404 - Post Not Found"</h1>
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
fn BlogContent(post: BlogPostData) -> impl IntoView {
    let info = markdown::to_html_with_options(&post.content, &markdown::Options::gfm())
        .unwrap_or_default();
    view! {
        <div class="markdown-body card-hover rounded-xl p-8 space-y-6" inner_html=info >
                    </div>
    }
}
