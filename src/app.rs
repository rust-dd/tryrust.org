use crate::{instruction::Instruction, server::session::create_session, terminal::Terminal};
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
        <main class="flex overflow-auto flex-col w-screen md:flex-row md:items-center md:h-screen bg-[#1e1e1e]">
            <div class="flex z-50 flex-row gap-2 items-center py-1 px-2 mt-8 ml-8 rounded-lg md:absolute md:top-4 md:left-8 md:mt-0 md:ml-0">
                <img src="rust_color.png" width=56 height=56/>
                <span class="text-4xl font-extrabold text-[#c6c6c6]">tryrust.org</span>
            </div>
            <div class="flex overflow-auto flex-col gap-4 justify-center items-center p-8 w-full md:flex-row md:gap-0 bg-custom-radial">
                <Terminal/>
                <Instruction/>
            </div>
        </main>
    }
}
