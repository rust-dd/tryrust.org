use crate::{
    context::{Exercises, Progress},
    instruction,
    server::session::create_session,
    terminal,
};
use chrono::{Datelike, Utc};
use leptos::*;
use leptos_meta::*;
use leptos_use::{storage::use_session_storage, utils::FromToStringCodec};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(Progress::default());
    provide_context(Exercises::default());
    let (_, set_state, ..) = use_session_storage::<String, FromToStringCodec>("session_id");

    create_effect(move |_| {
        spawn_local(async move {
            let session_id = create_session().await.unwrap();
            set_state(session_id);
        })
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/tryrust.css" />
        <main class="flex overflow-auto flex-col w-screen md:flex-row md:items-center md:h-screen bg-[#1e1e1e]">
            <div class="flex z-50 flex-row gap-2 items-center py-1 px-2 mt-8 ml-8 rounded-lg md:absolute md:top-4 md:left-8 md:mt-0 md:ml-0">
                <img src="rust_color.png" width=56 height=56 />
                <span class="text-4xl font-extrabold text-[#c6c6c6]">tryrust.org</span>
            </div>
            <div class="flex overflow-auto flex-col gap-4 justify-center items-center p-8 w-full md:flex-row md:gap-0 bg-custom-radial">
                <terminal::Component />
                <instruction::Component />
            </div>
            <div class="mb-4 text-center md:absolute md:right-0 md:left-0 md:bottom-4">
                <p class="text-gray-400">
                    Powered by <a href="https://github.com/rust-dd" class="text-[#ffbd2e]">
                        Rust-DD
                    </a> {" © "} {Utc::now().year()}
                </p>
            </div>
        </main>
    }
}
