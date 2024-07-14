use crate::{instruction::Instruction, server::session::create_session, terminal::Terminal};
use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_use::{storage::use_session_storage, utils::FromToStringCodec};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (_, set_state, ..) = use_session_storage::<String, FromToStringCodec>("session_id");

    create_effect(move |_| {
        spawn_local(async move {
            let session_id = create_session().await.unwrap();
            set_state(session_id);
        })
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/tryrust.css"/>
        <main class="w-screen md:h-screen md:items-center flex flex-col md:flex-row bg-[#1e1e1e] overflow-auto">
            <div class="md:absolute z-50 md:top-4 md:left-8 mt-8 ml-8 md:ml-0 md:mt-0 flex flex-row items-center gap-2 px-2 py-1 rounded-lg">
                <img src="rust_color.png" width=56 height=56 />
                <span class="font-extrabold text-4xl text-[#c6c6c6]">tryrust.org</span>
            </div>
            <div class="flex flex-col gap-4 w-full bg-custom-radial md:gap-0 md:flex-row p-8 overflow-auto items-center justify-center">
                <Terminal />
                <Instruction />
            </div>
        </main>
    }
}
