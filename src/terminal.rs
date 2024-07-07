use html::Input;
use leptos::*;
use leptos_use::{storage::use_session_storage, use_event_listener, utils::FromToStringCodec};

use crate::server::compile::compile;

#[component]
pub fn Terminal() -> impl IntoView {
    let commands = create_rw_signal(Vec::<String>::new());
    let responses = create_rw_signal(Vec::<String>::new());
    let (code, set_code) = create_signal(String::new());
    let (stale, set_stale) = create_signal(true);
    let input_ref = create_node_ref::<Input>();
    let (session_id, ..) = use_session_storage::<String, FromToStringCodec>("session_id");
    let compile_code = create_action(move |session_id: &String| {
        set_stale(false);
        let session_id = session_id.clone();
        async move {
            let response = compile(session_id.clone())
                .await
                .expect("Failed to compile");
            responses.update(|prev| {
                prev.push(response);
            });
            set_stale(true);
        }
    });

    let _ = use_event_listener(input_ref, ev::keydown, move |e| {
        if !stale() {
            return;
        }

        if e.key() == "Enter" {
            commands.update(|prev| {
                prev.push(code());
            });
            compile_code.dispatch(session_id());
        }
    });

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
                <div class="flex items-center">
                <span>$</span>
                 <input
                    _ref=input_ref
                    name="code"
                    r#type="text"
                    class="w-full px-2 py-1 bg-transparent border-none outline-none text-[#c6c6c6]"
                    placeholder="Type a command..."
                    value={code}
                    on:input=move |e| set_code(event_target_value(&e))
                    class="ml-2 animate-blink"
                     />
                </div>
            </div>
            <div class="absolute right-4 bottom-4">
                <Show when=stale fallback=move || view! {<span class="ml-2 text-white">Compiling...</span>}>
                    <div />
                </Show>
            </div>
        </div>
    }
}
