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
        <main class="flex flex-col gap-4 md:gap-0 md:flex-row p-8 overflow-auto items-center justify-center w-screen h-screen bg-[#1e1e1e]">
            <Terminal />
            <Instruction />
        </main>
    }
}
