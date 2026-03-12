use dioxus::prelude::*;

use crate::exercises::EXERCISES;
use crate::server::compile;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalEntry {
    Input(String),
    Success(String),
    Error(String),
    System(String),
}

#[component]
pub fn Terminal(
    session_id: Signal<String>,
    exercise_idx: Signal<usize>,
    step_idx: Signal<usize>,
    code_input: Signal<String>,
) -> Element {
    let mut history: Signal<Vec<TerminalEntry>> = use_signal(Vec::new);
    let mut is_compiling = use_signal(|| false);
    let mut cmd_history: Signal<Vec<String>> = use_signal(Vec::new);
    let mut history_pos: Signal<Option<usize>> = use_signal(|| None);

    let submit = move |()| async move {
        let code = code_input.read().trim().to_string();
        if code.is_empty() || *is_compiling.read() {
            return;
        }

        // Add to command history
        cmd_history.write().push(code.clone());
        history_pos.set(None);

        // Handle "clear" command
        if code.to_lowercase() == "clear" {
            history.write().clear();
            code_input.set(String::new());
            return;
        }

        let sid = session_id.read().clone();
        if sid.is_empty() {
            return;
        }

        history.write().push(TerminalEntry::Input(code.clone()));
        code_input.set(String::new());
        is_compiling.set(true);

        match compile(sid, code.clone()).await {
            Ok(response) => {
                let trimmed = response.trim().to_string();
                let is_error = trimmed.contains("error");
                if is_error {
                    history.write().push(TerminalEntry::Error(trimmed));
                } else {
                    if !trimmed.is_empty() {
                        history.write().push(TerminalEntry::Success(trimmed));
                    } else {
                        history.write().push(TerminalEntry::Success("OK".into()));
                    }
                    // Check exercise progress
                    let ex_i = *exercise_idx.read();
                    let st_i = *step_idx.read();
                    if ex_i < EXERCISES.len() {
                        let exercise = &EXERCISES[ex_i];
                        if st_i < exercise.steps.len() && code == exercise.steps[st_i].code {
                            if st_i + 1 < exercise.steps.len() {
                                step_idx.set(st_i + 1);
                            } else {
                                step_idx.set(0);
                                exercise_idx.set(ex_i + 1);
                                history.write().push(TerminalEntry::System(
                                    format!("Exercise complete: {}", exercise.title),
                                ));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                history
                    .write()
                    .push(TerminalEntry::Error(format!("{e}")));
            }
        }

        is_compiling.set(false);
        document::eval("setTimeout(()=>{let e=document.getElementById('term-out');if(e)e.scrollTop=e.scrollHeight;document.getElementById('code-input')?.focus()},30)");
    };

    let compiling = *is_compiling.read();

    rsx! {
        div { class: "flex flex-col h-full bg-[#060606]",

            // Terminal output
            div {
                id: "term-out",
                class: "flex-1 overflow-y-auto p-4 font-mono text-[13px] leading-relaxed",

                // Welcome
                if history.read().is_empty() {
                    div { class: "flex flex-col gap-2 mb-4 fade-in",
                        div { class: "flex items-center gap-2 mb-1",
                            span { class: "text-[#fbdc8ecc] text-lg font-bold", ">" }
                            span { class: "text-zinc-400 font-semibold", "Try Rust" }
                            span { class: "text-zinc-700 text-[11px]", "v1.0" }
                        }
                        span { class: "text-zinc-600 text-[12px]", "Interactive Rust tutorial in your browser." }
                        span { class: "text-zinc-700 text-[12px]", "Click a code snippet on the right, or type below." }
                        div { class: "flex gap-3 mt-1",
                            span { class: "text-zinc-700 text-[11px]", "arrow-up/down history" }
                            span { class: "text-zinc-800", "\u{00b7}" }
                            span { class: "text-zinc-700 text-[11px]", "\"clear\" to reset" }
                        }
                    }
                }

                for (i, entry) in history.read().iter().enumerate() {
                    match entry {
                        TerminalEntry::Input(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2 py-0.5",
                                span { class: "text-[#fbdc8eb3] select-none shrink-0 font-bold", ">" }
                                span { class: "text-zinc-200", "{t}" }
                            }
                        },
                        TerminalEntry::Success(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2 py-0.5",
                                span { class: "text-[#4cf490b3] select-none shrink-0", "~" }
                                span { class: "text-[#4cf490b3]", "{t}" }
                            }
                        },
                        TerminalEntry::Error(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2 py-0.5",
                                span { class: "text-[#ff4c4cb3] select-none shrink-0", "!" }
                                span { class: "text-[#ff4c4cb3]", "{t}" }
                            }
                        },
                        TerminalEntry::System(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2 py-0.5 celebrate",
                                span { class: "text-[#fbdc8eb3] select-none shrink-0", "*" }
                                span { class: "text-[#fbdc8ecc] font-medium", "{t}" }
                            }
                        },
                    }
                }
            }

            // Input bar
            div { class: "flex items-center gap-2 px-4 py-2.5 border-t border-white/[0.06] bg-white/[0.02]",
                span { class: "text-[#fbdc8e99] font-mono text-sm select-none shrink-0 font-bold", ">" }
                input {
                    id: "code-input",
                    r#type: "text",
                    autocomplete: "off",
                    spellcheck: "false",
                    autofocus: true,
                    disabled: compiling,
                    class: "flex-1 bg-transparent outline-none text-zinc-200 font-mono text-[13px] placeholder:text-zinc-700 caret-[#fbdc8e]",
                    placeholder: if compiling { "Compiling..." } else { "Type Rust code..." },
                    value: "{code_input}",
                    oninput: move |e| { code_input.set(e.value()); },
                    onkeydown: move |e| {
                        match e.key() {
                            Key::Enter => {
                                let val = code_input.read().clone();
                                if !val.trim().is_empty() && !compiling {
                                    spawn(submit(()));
                                }
                            }
                            Key::ArrowUp => {
                                e.prevent_default();
                                let hist = cmd_history.read();
                                if hist.is_empty() { return; }
                                let pos = match *history_pos.read() {
                                    Some(p) if p > 0 => p - 1,
                                    Some(0) => 0,
                                    None => hist.len() - 1,
                                    _ => 0,
                                };
                                code_input.set(hist[pos].clone());
                                drop(hist);
                                history_pos.set(Some(pos));
                            }
                            Key::ArrowDown => {
                                e.prevent_default();
                                let pos = *history_pos.read();
                                if let Some(p) = pos {
                                    let hist = cmd_history.read();
                                    if p + 1 < hist.len() {
                                        code_input.set(hist[p + 1].clone());
                                        drop(hist);
                                        history_pos.set(Some(p + 1));
                                    } else {
                                        drop(hist);
                                        code_input.set(String::new());
                                        history_pos.set(None);
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                }
                if compiling {
                    div { class: "w-3.5 h-3.5 border-[1.5px] border-[#fbdc8e4d] border-t-[#fbdc8e] rounded-full animate-spin shrink-0" }
                }
            }
        }
    }
}
