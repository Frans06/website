use leptos::{ev, prelude::*};
use leptos_router::components::A;

#[island]
pub fn NavigationBar() -> impl IntoView {
    let (is_mobile_open, set_mobile_open) = signal(false);
    let (is_scrolled, set_scrolled) = signal(false);

    // Scroll effect
    let handle = window_event_listener(ev::scroll, move |_| {
        set_scrolled.set(window().scroll_y().unwrap_or(0.0) > 50.0);
    });
    on_cleanup(move || handle.remove());

    view! {
        <nav class=move || format!(
            "fixed top-0 left-0 right-0 z-50 transition-all duration-300 {}",
            if is_scrolled.get() { "nav-blur border-b border-purple-500/20" } else { "" }
        )>
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo
                    <div class="flex-shrink-0">
                        <A href="/">
            <p class="orbitron text-2xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-500">
                            "FRN"
            </p>
                        </A>
                    </div>

                    // Desktop Navigation
                    <div class="hidden md:block">
                        <div class="ml-10 flex items-baseline space-x-8">
                            <A href="/" >
            <p class="nav-link px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                                "Home"
            </p>
                            </A>
                            <a href="/#about" class="nav-link px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                                "About"
                            </a>
                            <a href="/#experience" class="nav-link px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                                "Experience"
                            </a>
                                <A href="/blog">
                <p class="nav-link px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                                "Blog"
            </p>
                            </A>
                            <a href="/#contact" class="nav-link px-3 py-2 rounded-md text-sm font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                                "Contact"
                            </a>
                        </div>
                    </div>

                    // Mobile menu button
                    <div class="md:hidden">
                        <button
                            on:click=move |_| set_mobile_open.update(|open| *open = !*open)
                            class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-cyan-400 focus:outline-none transition-colors"
                        >
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                            </svg>
                        </button>
                    </div>
                </div>

                // Mobile Navigation Menu
                <div class=move || format!(
                    "md:hidden transition-all duration-300 {}",
                    if is_mobile_open.get() { "block" } else { "hidden" }
                )>
                    <div class="px-2 pt-2 pb-3 space-y-1 sm:px-3 nav-blur rounded-lg mt-2">
                        <A href="/" >
                <p class="block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                            "Home"
                </p>
                        </A>
                        <a href="/#about" class="block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                            "About"
                        </a>
                        <a href="/#experience" class="block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                            "Experience"
                        </a>
                        <A href="/blog">
                <p class="block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                            "Blog"
                </p>
                        </A>
                        <a href="/#contact" class="block px-3 py-2 rounded-md text-base font-medium text-gray-300 hover:text-cyan-400 transition-colors">
                            "Contact"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
