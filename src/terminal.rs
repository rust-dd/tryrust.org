use leptos::*;

#[component]
pub fn Terminal() -> impl IntoView {
    let (code, set_code) = create_signal(String::new());
    view! {
        <div class="w-[600px] h-[400px] bg-[#2b2b2b] rounded-l-xl overflow-hidden">
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
                    r#type="text"
                    class="w-full px-2 py-1 bg-transparent border-none outline-none text-[#c6c6c6]"
                    placeholder="Type a command..."
                    value={code}
                    on:input=move |e| set_code(event_target_value(&e))
                    class="ml-2 animate-blink"
                     />
                </div>
            </div>
        </div>
    }
}
