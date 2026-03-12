use dioxus::prelude::*;

mod exercises;
mod instruction;
mod playground;
mod server;
mod terminal;

use exercises::EXERCISES;
use instruction::Instruction;
use playground::Playground;
use server::create_session;
use terminal::Terminal;

static MAIN_CSS: Asset = asset!("/assets/tailwind.css");
static FAVICON: Asset = asset!("/public/favicon.ico");
static RUST_LOGO: Asset = asset!("/public/rust_color.png");

const CATEGORIES: &[&str] = &["Basics", "Intermediate", "Advanced", "Projects"];

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut session_id = use_signal(|| String::new());
    let mut exercise_idx = use_signal(|| 0usize);
    let mut step_idx = use_signal(|| 0usize);
    let mut active_tab = use_signal(|| 0u8); // mobile: 0=learn, 1=code
    let code_input = use_signal(|| String::new());
    let mut sidebar_open = use_signal(|| false);
    let mut playground_mode = use_signal(|| false);
    let mut progress_loaded = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            if let Ok(id) = create_session().await {
                session_id.set(id);
            }
        });
    });

    // Load progress from localStorage
    use_effect(move || {
        spawn(async move {
            let eval = document::eval("localStorage.getItem('tryrust_progress')");
            if let Ok(val) = eval.await {
                if let Some(s) = val.as_str() {
                    if let Ok(idx) = s.parse::<usize>() {
                        if idx > 0 && idx <= EXERCISES.len() {
                            exercise_idx.set(idx);
                        }
                    }
                }
            }
            progress_loaded.set(true);
        });
    });

    // Save progress to localStorage
    use_effect(move || {
        let idx = *exercise_idx.read();
        let loaded = *progress_loaded.read();
        if loaded {
            document::eval(&format!("localStorage.setItem('tryrust_progress','{}')", idx));
        }
    });

    let tab = *active_tab.read();
    let ex_i = *exercise_idx.read();
    let total = EXERCISES.len();
    let completed = ex_i;
    let show_sidebar = *sidebar_open.read();

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

        div { class: "h-[100dvh] flex flex-col bg-[#0d0d0e] text-white overflow-hidden",

            // Progress bar - top of page
            div { class: "w-full h-0.5 bg-[#202126] flex-shrink-0",
                div {
                    class: "h-full bg-gradient-to-r from-[#4cf490] to-[#8a38f5] transition-all duration-500",
                    style: "width: {((completed as f64 / total as f64) * 100.0) as u32}%",
                }
            }

            // Top bar
            header { class: "flex items-center justify-between px-4 py-2 border-b border-[#202126] flex-shrink-0",
                div { class: "flex items-center gap-3",
                    // Menu button (toggles sidebar)
                    button {
                        class: "hidden md:flex w-7 h-7 items-center justify-center rounded-md hover:bg-white/[0.06] transition-colors text-zinc-500 hover:text-zinc-300",
                        onclick: move |_| { sidebar_open.set(!show_sidebar); },
                        svg { width: "16", height: "16", view_box: "0 0 16 16", fill: "currentColor",
                            path { d: "M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 010 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 010 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 000 1.5h12.5a.75.75 0 000-1.5H1.75z" }
                        }
                    }
                    img { src: RUST_LOGO, width: 24, height: 24, alt: "Rust logo" }
                    span { class: "text-sm font-semibold text-zinc-300", "Try Rust" }

                    // Mode toggle
                    div { class: "hidden md:flex items-center ml-1 bg-[#202126] rounded-md p-0.5",
                        button {
                            class: if !*playground_mode.read() {
                                "px-2 py-0.5 text-[11px] font-medium rounded text-white bg-white/[0.08]"
                            } else {
                                "px-2 py-0.5 text-[11px] font-medium rounded text-zinc-500 hover:text-zinc-300 transition-colors"
                            },
                            onclick: move |_| { playground_mode.set(false); },
                            "Exercises"
                        }
                        button {
                            class: if *playground_mode.read() {
                                "px-2 py-0.5 text-[11px] font-medium rounded text-white bg-white/[0.08]"
                            } else {
                                "px-2 py-0.5 text-[11px] font-medium rounded text-zinc-500 hover:text-zinc-300 transition-colors"
                            },
                            onclick: move |_| { playground_mode.set(true); },
                            "Playground"
                        }
                    }
                }

                // Right group
                div { class: "flex items-center gap-2",
                    a {
                        href: "https://github.com/rust-dd/tryrust.org",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "w-7 h-7 flex items-center justify-center rounded-md hover:bg-white/[0.06] transition-colors text-zinc-500 hover:text-zinc-300",
                        aria_label: "GitHub",
                        svg {
                            width: "18", height: "18", view_box: "0 0 24 24", fill: "currentColor",
                            path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                        }
                    }
                }
            }

            // Mobile tabs
            div { class: "md:hidden flex flex-shrink-0 border-b border-[#202126]",
                button {
                    class: if tab == 0 && !*playground_mode.read() {
                        "flex-1 py-2 text-xs font-medium text-[#4cf490] border-b border-[#4cf490]"
                    } else {
                        "flex-1 py-2 text-xs font-medium text-zinc-600"
                    },
                    onclick: move |_| { active_tab.set(0); playground_mode.set(false); },
                    "Learn"
                }
                button {
                    class: if tab == 1 {
                        "flex-1 py-2 text-xs font-medium text-[#4cf490] border-b border-[#4cf490]"
                    } else {
                        "flex-1 py-2 text-xs font-medium text-zinc-600"
                    },
                    onclick: move |_| active_tab.set(1),
                    "Terminal"
                }
                button {
                    class: if tab == 0 && *playground_mode.read() {
                        "flex-1 py-2 text-xs font-medium text-[#4cf490] border-b border-[#4cf490]"
                    } else {
                        "flex-1 py-2 text-xs font-medium text-zinc-600"
                    },
                    onclick: move |_| { active_tab.set(0); playground_mode.set(true); },
                    "Playground"
                }
            }

            // Main layout
            div { class: "flex-1 flex min-h-0",

                // Sidebar - course outline (desktop only)
                if show_sidebar {
                    div { class: "hidden md:flex md:flex-col w-56 border-r border-[#202126] bg-[#141416] overflow-y-auto shrink-0",
                        div { class: "p-3",
                            for cat in CATEGORIES.iter() {
                                {
                                    let cat_name = *cat;
                                    let cat_exercises: Vec<(usize, &crate::exercises::Exercise)> = EXERCISES
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, e)| e.category == cat_name)
                                        .collect();

                                    let dot_color = match cat_name {
                                        "Basics" => "bg-[#4cf490]",
                                        "Intermediate" => "bg-[#02befa]",
                                        "Advanced" => "bg-[#a880ff]",
                                        _ => "bg-[#fbdc8e]",
                                    };

                                    rsx! {
                                        div { class: "mb-3",
                                            // Category header
                                            div { class: "flex items-center gap-1.5 mb-1 px-2",
                                                div { class: "w-1.5 h-1.5 rounded-full {dot_color}" }
                                                span { class: "text-[10px] font-semibold uppercase tracking-wider text-zinc-500", "{cat_name}" }
                                            }
                                            // Exercise list
                                            for (idx, ex) in cat_exercises.iter() {
                                                {
                                                    let exercise_index = *idx;
                                                    let is_current = exercise_index == ex_i;
                                                    let is_done = exercise_index < ex_i;
                                                    rsx! {
                                                        button {
                                                            class: if is_current {
                                                                "w-full text-left px-2 py-1 rounded text-[12px] text-white bg-white/[0.06] font-medium truncate"
                                                            } else if is_done {
                                                                "w-full text-left px-2 py-1 rounded text-[12px] text-zinc-500 hover:text-zinc-300 hover:bg-white/[0.03] transition-colors truncate"
                                                            } else {
                                                                "w-full text-left px-2 py-1 rounded text-[12px] text-zinc-600 hover:text-zinc-400 hover:bg-white/[0.03] transition-colors truncate"
                                                            },
                                                            onclick: move |_| {
                                                                exercise_idx.set(exercise_index);
                                                                step_idx.set(0);
                                                            },
                                                            if is_done {
                                                                span { class: "text-[#4cf490] mr-1", "\u{2713}" }
                                                            }
                                                            "{ex.title}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Footer
                        div { class: "mt-auto p-3 border-t border-[#202126]",
                            div { class: "flex items-center justify-between mb-1",
                                span { class: "text-[10px] text-zinc-600 tabular-nums", "{completed}/{total} completed" }
                                if completed > 0 {
                                    button {
                                        class: "text-[10px] text-zinc-700 hover:text-zinc-400 transition-colors",
                                        title: "Reset all progress",
                                        onclick: move |_| {
                                            exercise_idx.set(0);
                                            step_idx.set(0);
                                            document::eval("localStorage.removeItem('tryrust_progress')");
                                        },
                                        "Reset"
                                    }
                                }
                            }
                            span { class: "text-[10px] text-zinc-700",
                                "Powered by "
                                a { href: "https://github.com/rust-dd", target: "_blank", class: "text-zinc-600 hover:text-[#4cf490] transition-colors", "rust-dd" }
                            }
                        }
                    }
                }

                // Main content area
                div { class: "flex-1 flex flex-col md:flex-row min-h-0",

                    // Terminal panel (left)
                    div {
                        class: if tab == 1 {
                            "flex-1 flex flex-col min-h-0 md:flex"
                        } else {
                            "hidden md:flex md:flex-1 md:flex-col md:min-h-0"
                        },
                        Terminal { session_id, exercise_idx, step_idx, code_input }
                    }

                    // Right panel: exercise catalog or playground
                    div {
                        class: if tab == 0 {
                            "flex-1 flex flex-col min-h-0 md:flex md:w-[420px] md:max-w-[420px] md:shrink-0 md:flex-none md:border-l md:border-[#202126]"
                        } else {
                            "hidden md:flex md:w-[420px] md:max-w-[420px] md:shrink-0 md:flex-none md:flex-col md:min-h-0 md:border-l md:border-[#202126]"
                        },
                        if *playground_mode.read() {
                            Playground { session_id }
                        } else {
                            Instruction { exercise_idx, step_idx, code_input }
                        }
                    }
                }
            }

            section { class: "sr-only",
                h1 { "Learn Rust Programming with Try Rust" }
                p { "Try Rust is a free, browser-based interactive tutorial to help you learn Rust programming." }
            }
        }
    }
}
