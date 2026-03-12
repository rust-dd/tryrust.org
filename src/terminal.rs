use dioxus::prelude::*;

use crate::exercises::EXERCISES;
use crate::server::compile;

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalEntry {
    Input(String),
    Success(String),
    Error(String),
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

    let submit = move |()| async move {
        let code = code_input.read().trim().to_string();
        if code.is_empty() || *is_compiling.read() {
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
        div { class: "flex flex-col h-full bg-[#0c0c0c]",

            // Terminal output
            div {
                id: "term-out",
                class: "flex-1 overflow-y-auto p-4 font-mono text-[13px] leading-6",

                // Welcome
                if history.read().is_empty() {
                    div { class: "text-zinc-600 mb-2",
                        "// Welcome to Try Rust. Click a code snippet or type below."
                    }
                }

                for (i, entry) in history.read().iter().enumerate() {
                    match entry {
                        TerminalEntry::Input(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2",
                                span { class: "text-orange-400 select-none shrink-0", ">" }
                                span { class: "text-zinc-300", "{t}" }
                            }
                        },
                        TerminalEntry::Success(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2",
                                span { class: "text-emerald-400 select-none shrink-0", "~" }
                                span { class: "text-emerald-300/80", "{t}" }
                            }
                        },
                        TerminalEntry::Error(t) => rsx! {
                            div { key: "{i}", class: "flex gap-2",
                                span { class: "text-red-400 select-none shrink-0", "!" }
                                span { class: "text-red-400/80", "{t}" }
                            }
                        },
                    }
                }
            }

            // Input bar
            div { class: "flex items-center gap-2 px-4 py-3 border-t border-white/10 bg-[#111]",
                span { class: "text-orange-400 font-mono text-sm select-none shrink-0", ">" }
                input {
                    id: "code-input",
                    r#type: "text",
                    autocomplete: "off",
                    spellcheck: "false",
                    autofocus: true,
                    disabled: compiling,
                    class: "flex-1 bg-transparent outline-none text-zinc-200 font-mono text-[13px] placeholder:text-zinc-700 caret-orange-400",
                    placeholder: if compiling { "Compiling..." } else { "Type Rust code here..." },
                    value: "{code_input}",
                    oninput: move |e| { code_input.set(e.value()); },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            let val = code_input.read().clone();
                            if !val.trim().is_empty() && !compiling {
                                spawn(submit(()));
                            }
                        }
                    },
                }
                if compiling {
                    div { class: "w-4 h-4 border-2 border-orange-400/40 border-t-orange-400 rounded-full animate-spin shrink-0" }
                }
            }
        }
    }
}
