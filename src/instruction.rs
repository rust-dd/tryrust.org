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
        div { class: "flex flex-col h-full bg-[#0f0f11]",

            // Content
            div { class: "flex-1 overflow-y-auto p-5 md:p-6",
                if ex_i >= total {
                    // Completion
                    div { class: "flex flex-col gap-6",
                        div { class: "flex flex-col gap-2",
                            span { class: "text-4xl", "🎉" }
                            h2 { class: "text-2xl font-bold", "You did it!" }
                            p { class: "text-zinc-400 text-sm leading-relaxed",
                                "You've completed all exercises. Keep going with these resources:"
                            }
                        }
                        div { class: "flex flex-col gap-1.5",
                            LinkItem { href: "https://doc.rust-lang.org/book/", text: "The Rust Book" }
                            LinkItem { href: "https://www.rust-lang.org/", text: "rust-lang.org" }
                            LinkItem { href: "https://docs.rs/", text: "docs.rs" }
                            LinkItem { href: "https://crates.io/", text: "crates.io" }
                            LinkItem { href: "https://users.rust-lang.org/", text: "Rust Forum" }
                            LinkItem { href: "https://rust-lang.github.io/async-book/", text: "Async Book" }
                        }
                    }
                } else {
                    // Exercise
                    {
                        let exercise = &EXERCISES[ex_i];
                        rsx! {
                            div { class: "flex flex-col gap-5",
                                // Category + Title row
                                div { class: "flex flex-col gap-1.5",
                                    div { class: "flex items-center gap-2",
                                        span {
                                            class: match exercise.category {
                                                "Basics" => "text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20",
                                                "Intermediate" => "text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full bg-blue-500/10 text-blue-400 border border-blue-500/20",
                                                "Advanced" => "text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full bg-purple-500/10 text-purple-400 border border-purple-500/20",
                                                _ => "text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full bg-orange-500/10 text-orange-400 border border-orange-500/20",
                                            },
                                            "{exercise.category}"
                                        }
                                        span { class: "text-xs text-zinc-600 tabular-nums ml-auto shrink-0", "{ex_i + 1} / {total}" }
                                    }
                                    h2 { class: "text-xl font-bold", "{exercise.title}" }
                                }

                                // Progress bar
                                div { class: "w-full h-1 rounded-full bg-zinc-800 overflow-hidden",
                                    div {
                                        class: "h-full rounded-full bg-gradient-to-r from-orange-500 to-amber-400 transition-all duration-500",
                                        style: "width: {((st_i as f64) / (exercise.steps.len() as f64) * 100.0) as u32}%",
                                    }
                                }

                                p { class: "text-zinc-500 text-sm leading-relaxed", "{exercise.description}" }

                                // Steps
                                div { class: "flex flex-col gap-2",
                                    for (i, step) in exercise.steps.iter().enumerate() {
                                        {
                                            let done = i < st_i;
                                            let current = i == st_i;
                                            let code = step.code;
                                            let display_code = step.display.unwrap_or(step.code);

                                            rsx! {
                                                div {
                                                    key: "{ex_i}-{i}",
                                                    class: if done {
                                                        "group rounded-lg border border-emerald-500/20 bg-emerald-500/[0.04] p-3"
                                                    } else if current {
                                                        "group rounded-lg border border-orange-500/30 bg-orange-500/[0.04] p-3"
                                                    } else {
                                                        "group rounded-lg border border-white/[0.04] p-3 opacity-40"
                                                    },

                                                    // Label
                                                    div { class: "flex items-center gap-2 mb-2",
                                                        span {
                                                            class: if done {
                                                                "text-[11px] font-bold text-emerald-400"
                                                            } else if current {
                                                                "text-[11px] font-bold text-orange-400"
                                                            } else {
                                                                "text-[11px] text-zinc-600"
                                                            },
                                                            if done { "✓ " } else { "" }
                                                            "{step.label}"
                                                        }
                                                    }

                                                    // Code block
                                                    div {
                                                        class: "rounded-md bg-black/40 hover:bg-black/60 cursor-pointer transition-colors px-3 py-2 overflow-x-auto",
                                                        onclick: move |_| {
                                                            code_input.set(code.to_string());
                                                            document::eval("setTimeout(()=>document.getElementById('code-input')?.focus(),20)");
                                                        },
                                                        code { class: "text-[12px] font-mono text-orange-300 whitespace-pre-wrap break-all", "{display_code}" }
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

            // Category nav bar
            div { class: "flex flex-col border-t border-white/10 shrink-0",
                // Category dots grouped
                div { class: "flex items-center gap-3 px-4 py-2 overflow-x-auto",
                    for cat in CATEGORIES.iter() {
                        {
                            let cat_name = *cat;
                            let cat_exercises: Vec<(usize, &crate::exercises::Exercise)> = EXERCISES
                                .iter()
                                .enumerate()
                                .filter(|(_, e)| e.category == cat_name)
                                .collect();

                            let cat_color = match cat_name {
                                "Basics" => "text-emerald-500",
                                "Intermediate" => "text-blue-500",
                                "Advanced" => "text-purple-500",
                                _ => "text-orange-500",
                            };

                            rsx! {
                                div { class: "flex items-center gap-1.5 shrink-0",
                                    span { class: "text-[9px] font-semibold uppercase tracking-wider {cat_color} mr-0.5", "{cat_name}" }
                                    for (idx, _ex) in cat_exercises.iter() {
                                        {
                                            let exercise_index = *idx;
                                            rsx! {
                                                button {
                                                    class: if exercise_index < ex_i {
                                                        "w-2 h-2 rounded-full bg-emerald-400 hover:scale-150 transition-transform cursor-pointer"
                                                    } else if exercise_index == ex_i {
                                                        "w-2.5 h-2.5 rounded-full bg-orange-400 hover:scale-125 transition-transform cursor-pointer ring-2 ring-orange-400/30"
                                                    } else {
                                                        "w-2 h-2 rounded-full bg-zinc-700 hover:bg-zinc-500 hover:scale-150 transition-all cursor-pointer"
                                                    },
                                                    onclick: move |_| {
                                                        exercise_idx.set(exercise_index);
                                                        step_idx.set(0);
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Prev / Next
                div { class: "flex items-center justify-between px-4 py-2 border-t border-white/5",
                    button {
                        class: "text-sm text-zinc-500 hover:text-white disabled:opacity-30 disabled:cursor-not-allowed transition-colors px-3 py-1 rounded hover:bg-white/5",
                        disabled: ex_i == 0,
                        onclick: move |_| {
                            let c = *exercise_idx.read();
                            if c > 0 {
                                exercise_idx.set(c - 1);
                                step_idx.set(0);
                            }
                        },
                        "← Prev"
                    }

                    if ex_i < total {
                        span { class: "text-xs text-zinc-600", "{EXERCISES[ex_i].category}" }
                    }

                    button {
                        class: "text-sm text-zinc-500 hover:text-white transition-colors px-3 py-1 rounded hover:bg-white/5",
                        onclick: move |_| {
                            let c = *exercise_idx.read();
                            if c < total {
                                exercise_idx.set(c + 1);
                            } else {
                                exercise_idx.set(0);
                            }
                            step_idx.set(0);
                        },
                        if ex_i >= total { "Restart →" } else { "Next →" }
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
            class: "flex items-center gap-2 text-sm text-zinc-400 hover:text-orange-400 transition-colors py-1.5 px-3 rounded-lg hover:bg-white/5",
            span { "→" }
            span { "{text}" }
        }
    }
}
