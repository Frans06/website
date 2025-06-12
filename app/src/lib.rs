use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

mod components;
mod pages;
mod posts;

use components::background::Background;

use crate::{
    components::navigation::NavigationBar,
    pages::{blog::BlogPage, blog_post::BlogPost, home::HomePage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Expert Fullstack Software Engineer specializing in modern web technologies"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/webpage.css"/>
        // sets the document title
        <Title text="Frans Ramirez Neyra - Software Engineer"/>

        <Meta name="description" content="Expert Fullstack Software Engineer specializing in modern web technologies"/>
        // content for this welcome page
        <Router>
            <Background />
            <NavigationBar/>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=path!("/") view=HomePage/>
                        <Route path=path!("/blog") view=BlogPage/>
                        <Route path=path!("/blog/:slug") view=BlogPost/>
                </Routes>
            </main>
        </Router>
    }
}
