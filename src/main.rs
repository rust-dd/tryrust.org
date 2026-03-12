use dioxus::prelude::*;

mod exercises;
mod instruction;
mod server;
mod terminal;

use instruction::Instruction;
use server::create_session;
use terminal::Terminal;

static MAIN_CSS: Asset = asset!("/assets/tailwind.css");
static FAVICON: Asset = asset!("/public/favicon.ico");
static RUST_LOGO: Asset = asset!("/public/rust_color.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut session_id = use_signal(|| String::new());
    let exercise_idx = use_signal(|| 0usize);
    let step_idx = use_signal(|| 0usize);
    let mut active_tab = use_signal(|| 0u8);
    let code_input = use_signal(|| String::new());

    use_effect(move || {
        spawn(async move {
            if let Ok(id) = create_session().await {
                session_id.set(id);
            }
        });
    });

    let tab = *active_tab.read();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Title { "Try Rust - Interactive Rust Tutorial | Learn Rust Online" }
        document::Meta { name: "description", content: "Learn Rust online with Try Rust — a free, interactive Rust tutorial in your browser. Write and run Rust code instantly with no setup required." }
        document::Meta { name: "keywords", content: "rust tutorial, learn rust, try rust online, rust programming, beginner rust course, rust exercises, interactive rust, rust playground" }
        document::Meta { name: "author", content: "rust-dd" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        document::Meta { property: "og:title", content: "Try Rust - Learn Rust in Your Browser" }
        document::Meta { property: "og:description", content: "Free interactive Rust programming course. Run Rust code in your browser and learn step by step." }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "https://tryrust.org" }
        document::Meta { property: "og:site_name", content: "Try Rust" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
        document::Meta { name: "twitter:title", content: "Try Rust - Interactive Rust Tutorial" }
        document::Meta { name: "twitter:description", content: "Learn Rust online for free. Interactive browser-based tutorial with real compilation." }
        document::Link { rel: "canonical", href: "https://tryrust.org" }
        document::Link { rel: "icon", href: FAVICON, r#type: "image/x-icon" }

        div { class: "h-[100dvh] flex flex-col bg-[#09090b] text-white overflow-hidden",

            // Header
            header { class: "relative flex items-center justify-between px-5 md:px-8 py-3 border-b border-white/10 flex-shrink-0 overflow-hidden",
                div { class: "flex items-center gap-3 z-10",
                    img { src: RUST_LOGO, width: 36, height: 36, alt: "Rust logo" }
                    span { class: "text-lg font-semibold tracking-tight", "Try Rust" }
                }

                a {
                    href: "https://github.com/rust-dd/tryrust.org",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "z-10 w-8 h-8 flex items-center justify-center rounded-lg hover:bg-white/10 transition-colors text-zinc-400 hover:text-white",
                    aria_label: "GitHub",
                    svg {
                        width: "20", height: "20", view_box: "0 0 24 24", fill: "currentColor",
                        path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                    }
                }
            }

            // Mobile tabs
            div { class: "md:hidden flex flex-shrink-0 bg-[#09090b]",
                button {
                    class: if tab == 0 { "flex-1 py-2.5 text-sm font-medium text-orange-400 border-b-2 border-orange-400" } else { "flex-1 py-2.5 text-sm font-medium text-zinc-500 border-b-2 border-transparent" },
                    onclick: move |_| active_tab.set(0),
                    "Code"
                }
                button {
                    class: if tab == 1 { "flex-1 py-2.5 text-sm font-medium text-orange-400 border-b-2 border-orange-400" } else { "flex-1 py-2.5 text-sm font-medium text-zinc-500 border-b-2 border-transparent" },
                    onclick: move |_| active_tab.set(1),
                    "Learn"
                }
            }

            // Main content - fills remaining space
            div { class: "flex-1 flex flex-col md:flex-row min-h-0",

                // Terminal
                div {
                    class: if tab == 0 { "flex-1 flex flex-col min-h-0 md:flex md:border-r md:border-white/10" } else { "hidden md:flex md:flex-1 md:flex-col md:min-h-0 md:border-r md:border-white/10" },
                    Terminal { session_id, exercise_idx, step_idx, code_input }
                }

                // Instructions
                div {
                    class: if tab == 1 { "flex-1 flex flex-col min-h-0 md:flex" } else { "hidden md:flex md:flex-1 md:flex-col md:min-h-0" },
                    Instruction { exercise_idx, step_idx, code_input }
                }
            }

            // Footer
            footer { class: "flex items-center justify-center py-2 border-t border-white/10 flex-shrink-0",
                span { class: "text-[11px] text-zinc-600",
                    "Powered by "
                    a { href: "https://github.com/rust-dd", target: "_blank", class: "text-zinc-500 hover:text-orange-400 transition-colors", "rust-dd" }
                }
            }

            section { class: "sr-only",
                h1 { "Learn Rust Programming with Try Rust" }
                p { "Try Rust is a free, browser-based interactive tutorial to help you learn Rust programming." }
            }
        }
    }
}
