use dioxus::prelude::*;

use crate::server::run_playground;

#[component]
pub fn Playground(session_id: Signal<String>) -> Element {
    let mut code = use_signal(|| {
        "fn main() {\n    println!(\"Hello, playground!\");\n}".to_string()
    });
    let mut output: Signal<Vec<(bool, String)>> = use_signal(Vec::new);
    let mut is_running = use_signal(|| false);

    let run_code = move || {
        let c = code.read().clone();
        let sid = session_id.read().clone();
        if sid.is_empty() || c.trim().is_empty() || *is_running.read() {
            return;
        }
        spawn(async move {
            is_running.set(true);
            match run_playground(sid, c).await {
                Ok(result) => {
                    let trimmed = result.trim().to_string();
                    let is_error = trimmed.contains("error");
                    if trimmed.is_empty() {
                        output.write().push((false, "OK".into()));
                    } else {
                        output.write().push((is_error, trimmed));
                    }
                }
                Err(e) => {
                    output.write().push((true, format!("{e}")));
                }
            }
            is_running.set(false);
        });
    };

    let running = *is_running.read();

    rsx! {
        div { class: "flex flex-col h-full bg-[#141416]",

            // Header
            div { class: "flex items-center justify-between px-4 py-2 border-b border-[#202126]",
                div { class: "flex items-center gap-2",
                    span { class: "text-[11px] font-semibold uppercase tracking-wider text-[#fbdc8e]", "Playground" }
                    span { class: "text-[10px] text-zinc-700", "Ctrl+Enter to run" }
                }
                div { class: "flex items-center gap-2",
                    button {
                        class: "text-[10px] text-zinc-600 hover:text-zinc-400 transition-colors",
                        onclick: move |_| { output.write().clear(); },
                        "Clear"
                    }
                    button {
                        class: if running {
                            "px-3 py-1 text-[11px] font-medium rounded bg-zinc-700/30 text-zinc-500 cursor-not-allowed"
                        } else {
                            "px-3 py-1 text-[11px] font-medium rounded bg-[#4cf49033] text-[#4cf490] hover:bg-[#4cf4904d] transition-colors"
                        },
                        disabled: running,
                        onclick: move |_| run_code(),
                        if running { "Running..." } else { "Run" }
                    }
                }
            }

            // Code editor
            div { class: "flex-1 min-h-0 overflow-hidden",
                textarea {
                    class: "w-full h-full bg-[#060606] text-zinc-200 font-mono text-[13px] leading-relaxed p-4 resize-none outline-none placeholder:text-zinc-700 caret-[#fbdc8e]",
                    spellcheck: "false",
                    placeholder: "Write your Rust code here...",
                    value: "{code}",
                    oninput: move |e| { code.set(e.value()); },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter
                            && (e.modifiers().contains(Modifiers::CONTROL)
                                || e.modifiers().contains(Modifiers::META))
                        {
                            run_code();
                        }
                    },
                }
            }

            // Output area
            if !output.read().is_empty() || running {
                div { class: "border-t border-[#202126] max-h-[40%] overflow-y-auto p-4 font-mono text-[13px] leading-relaxed bg-[#060606]",
                    for (i, (is_err, text)) in output.read().iter().enumerate() {
                        div { key: "{i}", class: "flex gap-2 py-0.5",
                            if *is_err {
                                span { class: "text-[#ff4c4cb3] select-none shrink-0", "!" }
                                span { class: "text-[#ff4c4cb3] whitespace-pre-wrap", "{text}" }
                            } else {
                                span { class: "text-[#4cf490b3] select-none shrink-0", "~" }
                                span { class: "text-[#4cf490b3] whitespace-pre-wrap", "{text}" }
                            }
                        }
                    }
                    if running {
                        div { class: "flex items-center gap-2 py-0.5",
                            div { class: "w-3 h-3 border-[1.5px] border-[#fbdc8e4d] border-t-[#fbdc8e] rounded-full animate-spin" }
                            span { class: "text-zinc-600 text-[12px]", "Compiling..." }
                        }
                    }
                }
            }
        }
    }
}
