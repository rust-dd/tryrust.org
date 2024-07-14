use std::collections::BTreeMap;

use html::Input;
use leptos::*;
use leptos_use::{storage::use_session_storage, use_event_listener, utils::FromToStringCodec};

use crate::server::compile::compile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum TerminalEvent {
    Code,
    Error,
    Success,
}

#[derive(Clone, Copy)]
pub struct CodeSetter(pub WriteSignal<String>);

#[derive(Clone, Copy)]
pub struct InputRef(pub ReadSignal<NodeRef<Input>>);

#[component]
pub fn Terminal() -> impl IntoView {
    let data = create_rw_signal(BTreeMap::<(usize, TerminalEvent), String>::new());
    let (code, set_code) = create_signal(String::new());
    provide_context(CodeSetter(set_code));
    let (r#ref, ..) = create_signal(create_node_ref::<Input>());
    let _ref = r#ref();
    provide_context(InputRef(r#ref));
    let (session_id, ..) = use_session_storage::<String, FromToStringCodec>("session_id");
    let compile_code = create_action(move |(session_id, code): &(String, String)| {
        let session_id = session_id.clone();
        let code = code.clone();
        data.update(|prev| {
            prev.insert((prev.len(), TerminalEvent::Code), code.clone());
        });

        set_code(Default::default());

        async move {
            let response = compile(session_id.clone(), code.clone())
                .await
                .expect("Failed to compile");

            if !response.is_empty() {
                data.update(|prev| {
                    let key = if response.contains("error") {
                        TerminalEvent::Error
                    } else {
                        TerminalEvent::Success
                    };
                    prev.insert((prev.len(), key), response);
                });
            }

            r#ref()
                .get()
                .expect("input_ref should be loaded by now")
                .scroll_into_view();
        }
    });

    let _ = use_event_listener(r#ref(), ev::keydown, move |e| {
        if compile_code.pending()() || code().is_empty() {
            return;
        }

        if e.key() == "Enter" {
            compile_code.dispatch((session_id(), code()));
        }
    });

    view! {
        <div class="flex flex-col max-w-[600px] w-full h-[400px] relative bg-[#2b2b2b] rounded-xl md:rounded-l-xl md:rounded-r-none">
            <div class="flex items-center rounded-t-xl justify-between px-4 py-2 bg-[#3c3c3c] border-b border-[#4d4d4d] md:rounded-tl-xl md:rounded-tr-none">
                <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-[#ff5f56]" />
                <div class="w-3 h-3 rounded-full bg-[#ffbd2e]" />
                <div class="w-3 h-3 rounded-full bg-[#27c93f]" />
                </div>
                <div class="text-[#9e9e9e] text-sm">Terminal</div>
                <div />
            </div>
            <div class="flex-1 px-4 pt-4 mb-8 font-mono text-[#c6c6c6] text-sm leading-relaxed overflow-auto">
                <For
                    each=move || data.get().into_iter()
                    key=|((idx, _), _)| *idx
                    children=|((_, r#type), code)| {
                            match r#type {
                                TerminalEvent::Error => {
                                    view! {<div class="flex items py-1 gap-4">
                                        <span class="text-[#ff5f56]">></span>
                                        <span>{code}</span>
                                    </div>
                                    }
                                }
                                TerminalEvent::Success => {
                                    view! {<div class="flex items py-1 gap-4">
                                        <span class="text-[#27c93f]">></span>
                                        <span>{code}</span>
                                    </div>
                                    }
                                }
                                TerminalEvent::Code => {
                                    view! {<div class="flex items py-1 gap-4">
                                        <span class="text-[#ffef5c]">$</span>
                                        <span>{code}</span>
                                    </div>
                                    }
                                }
                            }
                        }
                />
                <div class="flex items-center gap-2">
                    <span class="text-[#ffef5c]">$</span>
                    <input
                        _ref=_ref
                        name="code"
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
                <Show when=move || !compile_code.pending()() fallback=move || view! {<span class="ml-2 text-white text-sm">Compiling...</span>}>
                    <div />
                </Show>
            </div>
        </div>
    }
}
