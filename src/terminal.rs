use std::collections::BTreeMap;

use html::Input;
use leptos::*;
use leptos_use::{storage::use_session_storage, use_event_listener, utils::FromToStringCodec};

use crate::server::compile::compile;

#[component]
pub fn Terminal() -> impl IntoView {
    let data = create_rw_signal(BTreeMap::<usize, String>::new());
    let (code, set_code) = create_signal(String::new());
    let input_ref = create_node_ref::<Input>();
    let (session_id, ..) = use_session_storage::<String, FromToStringCodec>("session_id");
    let compile_code = create_action(move |(session_id, code): &(String, String)| {
        set_code(String::new());
        let session_id = session_id.clone();
        let code = code.clone();
        async move {
            let response = compile(session_id.clone(), code.clone())
                .await
                .expect("Failed to compile");

            if !response.is_empty() {
                data.update(|prev| {
                    prev.insert(prev.len(), response);
                });
            }
        }
    });

    let _ = use_event_listener(input_ref, ev::keydown, move |e| {
        if compile_code.pending()() || code().is_empty() {
            return;
        }

        if e.key() == "Enter" {
            data.update(|prev| {
                prev.insert(prev.len(), code());
            });
            compile_code.dispatch((session_id(), code()));
        }
    });

    // create_effect(move |_| log!("Terminal component mounted {:?}", data.get()));

    view! {
        <div class="w-[600px] h-[400px] relative bg-[#2b2b2b] rounded-l-xl overflow-hidden">
            <div class="flex items-center justify-between px-4 py-2 bg-[#3c3c3c] border-b border-[#4d4d4d]">
                <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-[#ff5f56]" />
                <div class="w-3 h-3 rounded-full bg-[#ffbd2e]" />
                <div class="w-3 h-3 rounded-full bg-[#27c93f]" />
                </div>
                <div class="text-[#9e9e9e] text-sm">Terminal</div>
                <div />
            </div>
            <div class="flex-1 p-4 font-mono text-[#c6c6c6] text-sm leading-relaxed overflow-auto">
                <For
                    each=move || data.get().into_iter()
                    key=|(idx, _)| *idx
                    children=|(_, item)| {
                        view! {<div class="flex items py-1 gap-4">
                            <span class="text-[#ffef5c]">$</span>
                            <span>{item}</span>
                        </div>
                        }
                    }
                />
                <div class="flex items-center gap-2">
                    <span class="text-[#ffef5c]">$</span>
                    <input
                        _ref=input_ref
                        name="code"
                        disabled=move || compile_code.pending()()
                        r#type="text"
                        autocomplete="off"
                        class="w-full py-1 px-2 bg-transparent border-none outline-none text-[#c6c6c6] caret-[#ffef5c]"
                        placeholder="Type a command..."
                        prop:value=move || code()
                        autofocus=true
                        on:input=move |e| set_code(event_target_value(&e))
                        />
                </div>
            </div>
            <div class="absolute right-4 bottom-4">
                <Show when=move || !compile_code.pending()() fallback=move || view! {<span class="ml-2 text-white">Compiling...</span>}>
                    <div />
                </Show>
            </div>
        </div>
    }
}
