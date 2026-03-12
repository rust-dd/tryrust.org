use dioxus::prelude::*;

use crate::exercises::EXERCISES;

const CATEGORIES: &[&str] = &["Basics", "Intermediate", "Advanced", "Projects"];

#[component]
pub fn Instruction(
    exercise_idx: Signal<usize>,
    step_idx: Signal<usize>,
    code_input: Signal<String>,
) -> Element {
    let ex_i = *exercise_idx.read();
    let st_i = *step_idx.read();
    let total = EXERCISES.len();

    rsx! {
        div { class: "flex flex-col h-full bg-[#0c0f16]",

            // Scrollable catalog
            div { class: "flex-1 overflow-y-auto",
                for cat in CATEGORIES.iter() {
                    {
                        let cat_name = *cat;
                        let cat_exercises: Vec<(usize, &crate::exercises::Exercise)> = EXERCISES
                            .iter()
                            .enumerate()
                            .filter(|(_, e)| e.category == cat_name)
                            .collect();

                        let (dot_color, text_color) = match cat_name {
                            "Basics" => ("bg-emerald-500", "text-emerald-500"),
                            "Intermediate" => ("bg-blue-500", "text-blue-500"),
                            "Advanced" => ("bg-purple-500", "text-purple-500"),
                            _ => ("bg-amber-500", "text-amber-500"),
                        };

                        // Count completed in category
                        let done_count = cat_exercises.iter().filter(|(idx, _)| *idx < ex_i).count();
                        let cat_total = cat_exercises.len();

                        rsx! {
                            div { class: "border-b border-[#1c2333]",
                                // Category header
                                div { class: "flex items-center gap-2 px-4 py-2.5",
                                    div { class: "w-2 h-2 rounded-full {dot_color}" }
                                    span { class: "text-[11px] font-semibold uppercase tracking-wider {text_color}", "{cat_name}" }
                                    span { class: "text-[10px] text-zinc-700 ml-auto tabular-nums", "{done_count}/{cat_total}" }
                                }

                                // Exercise items
                                div { class: "pb-1",
                                    for (idx, ex) in cat_exercises.iter() {
                                        {
                                            let exercise_index = *idx;
                                            let is_current = exercise_index == ex_i;
                                            let is_done = exercise_index < ex_i;

                                            rsx! {
                                                div {
                                                    // Exercise row - clickable
                                                    button {
                                                        class: if is_current {
                                                            "w-full text-left flex items-center gap-2.5 px-4 py-2 bg-white/[0.04]"
                                                        } else {
                                                            "w-full text-left flex items-center gap-2.5 px-4 py-1.5 hover:bg-white/[0.02] transition-colors"
                                                        },
                                                        onclick: move |_| {
                                                            exercise_idx.set(exercise_index);
                                                            step_idx.set(0);
                                                        },

                                                        // Status icon
                                                        if is_done {
                                                            div { class: "w-4 h-4 rounded-full bg-emerald-500/20 flex items-center justify-center shrink-0",
                                                                span { class: "text-[9px] text-emerald-400", "✓" }
                                                            }
                                                        } else if is_current {
                                                            div { class: "w-4 h-4 rounded-full border-2 border-amber-400 shrink-0",
                                                                div { class: "w-1.5 h-1.5 rounded-full bg-amber-400 m-auto mt-[3px]" }
                                                            }
                                                        } else {
                                                            div { class: "w-4 h-4 rounded-full border border-zinc-700 shrink-0" }
                                                        }

                                                        // Title
                                                        span {
                                                            class: if is_current {
                                                                "text-[13px] text-white font-medium truncate"
                                                            } else if is_done {
                                                                "text-[13px] text-zinc-500 truncate"
                                                            } else {
                                                                "text-[13px] text-zinc-600 truncate"
                                                            },
                                                            "{ex.title}"
                                                        }

                                                        // Step count
                                                        if is_current && ex.steps.len() > 1 {
                                                            span { class: "text-[10px] text-zinc-700 ml-auto tabular-nums shrink-0",
                                                                "{st_i}/{ex.steps.len()}"
                                                            }
                                                        }
                                                    }

                                                    // Expanded content for current exercise
                                                    if is_current && exercise_index < total {
                                                        {
                                                            let exercise = &EXERCISES[exercise_index];
                                                            rsx! {
                                                                div { class: "px-4 pb-3",
                                                                    // Description
                                                                    p { class: "text-[12px] text-zinc-500 leading-relaxed pl-6 mb-3",
                                                                        "{exercise.description}"
                                                                    }

                                                                    // Steps
                                                                    div { class: "flex flex-col gap-1.5 pl-2",
                                                                        for (i, step) in exercise.steps.iter().enumerate() {
                                                                            {
                                                                                let done = i < st_i;
                                                                                let current = i == st_i;
                                                                                let code = step.code;
                                                                                let display_code = step.display.unwrap_or(step.code);

                                                                                rsx! {
                                                                                    div {
                                                                                        key: "{exercise_index}-{i}",
                                                                                        class: if done {
                                                                                            "flex gap-2 items-start"
                                                                                        } else if current {
                                                                                            "flex gap-2 items-start"
                                                                                        } else {
                                                                                            "flex gap-2 items-start opacity-30"
                                                                                        },

                                                                                        // Step indicator line
                                                                                        div { class: "flex flex-col items-center pt-1 shrink-0 w-4",
                                                                                            if done {
                                                                                                div { class: "w-2 h-2 rounded-full bg-emerald-500/50" }
                                                                                            } else if current {
                                                                                                div { class: "w-2 h-2 rounded-full bg-amber-400" }
                                                                                            } else {
                                                                                                div { class: "w-1.5 h-1.5 rounded-full bg-zinc-700 mt-0.5" }
                                                                                            }
                                                                                        }

                                                                                        // Step content
                                                                                        div { class: "flex-1 min-w-0",
                                                                                            span {
                                                                                                class: if done {
                                                                                                    "text-[11px] text-emerald-500/70 block mb-1"
                                                                                                } else if current {
                                                                                                    "text-[11px] text-amber-400/90 font-medium block mb-1"
                                                                                                } else {
                                                                                                    "text-[11px] text-zinc-600 block mb-1"
                                                                                                },
                                                                                                "{step.label}"
                                                                                            }

                                                                                            div {
                                                                                                class: if current {
                                                                                                    "rounded bg-[#0d1117] hover:bg-[#111822] cursor-pointer transition-colors px-2.5 py-1.5 overflow-x-auto border border-[#1c2333]"
                                                                                                } else {
                                                                                                    "rounded bg-[#0d1117] hover:bg-[#111822] cursor-pointer transition-colors px-2.5 py-1.5 overflow-x-auto"
                                                                                                },
                                                                                                onclick: move |_| {
                                                                                                    code_input.set(code.to_string());
                                                                                                    document::eval("setTimeout(()=>document.getElementById('code-input')?.focus(),20)");
                                                                                                },
                                                                                                code { class: "text-[11px] font-mono text-amber-300/70 whitespace-pre-wrap break-all leading-relaxed", "{display_code}" }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Completion section
                if ex_i >= total {
                    div { class: "p-5",
                        div { class: "flex flex-col gap-4 items-center text-center pt-4",
                            div { class: "text-4xl", "🎉" }
                            h2 { class: "text-lg font-semibold", "All done!" }
                            p { class: "text-zinc-500 text-[13px] leading-relaxed max-w-[280px]",
                                "You've completed all exercises. Keep learning:"
                            }
                        }
                        div { class: "flex flex-col gap-0.5 mt-4",
                            LinkItem { href: "https://doc.rust-lang.org/book/", text: "The Rust Book" }
                            LinkItem { href: "https://www.rust-lang.org/", text: "rust-lang.org" }
                            LinkItem { href: "https://docs.rs/", text: "docs.rs" }
                            LinkItem { href: "https://crates.io/", text: "crates.io" }
                            LinkItem { href: "https://users.rust-lang.org/", text: "Rust Forum" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LinkItem(href: &'static str, text: &'static str) -> Element {
    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            rel: "noopener",
            class: "flex items-center gap-2 text-[13px] text-zinc-500 hover:text-amber-400 transition-colors py-1.5 px-3 rounded hover:bg-white/[0.03]",
            span { class: "text-zinc-700", "→" }
            span { "{text}" }
        }
    }
}
