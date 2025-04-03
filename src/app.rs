use crate::{
    context::{Exercises, Progress},
    instruction,
    server::session::create_session,
    terminal,
};
use chrono::{Datelike, Utc};
use codee::string::FromToStringCodec;
use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_use::storage::use_session_storage;

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
        <Title text="Try Rust - Rust Programming Experiment" />
        <Meta name="description" content="Learn Rust online with Try Rust — an interactive Rust tutorial in your browser. Start coding Rust instantly with no setup." />
        <Meta name="keywords" content="rust tutorial, learn rust, try rust online, rust programming, beginner rust course" />
        <Meta name="author" content="rust-dd" />
        <Meta name="robots" content="index, follow" />
        <Meta property="og:title" content="Try Rust - Learn Rust in Your Browser" />
        <Meta property="og:description" content="Interactive Rust programming course. Run Rust code in your browser and learn as you go." />
        <Meta property="og:type" content="website" />
        <Meta property="og:url" content="https://tryrust.org" />        

        <main class="flex overflow-auto flex-col w-screen md:flex-row md:items-center md:h-screen bg-[#1e1e1e]">
            <div class="flex z-50 w-full flex-row gap-2 items-center justify-between py-1 mt-8 px-8 rounded-lg md:absolute md:top-4 md:mt-0 md:ml-0">
                <div class="flex items-center gap-2">
                    <img src="rust_color.png" width=56 height=56 />
                    <span class="text-4xl font-extrabold text-[#c6c6c6]">Try Rust</span>
                </div>
                <a href="https://github.com/rust-dd/tryrust.org" target="_blank">
                    <Icon icon=i::IoLogoGithub class="size-8 text-white" />
                </a>
            </div>
            <div class="flex overflow-auto flex-col gap-4 justify-center items-center p-8 w-full md:flex-row md:gap-0 bg-custom-radial">
                <terminal::Component />
                <instruction::Component />

                <section class="sr-only">
                    <h1>Learn Rust with Try Rust</h1>
                    <p>
                        Try Rust is a free, browser-based interactive tutorial to help you learn Rust programming. 
                        No setup required. Just write and run Rust code directly in your browser.
                    </p>
                    <ul>
                        <li>Practice Rust syntax and concepts</li>
                        <li>Interactive coding exercises and instructions</li>
                        <li>Great for beginners and those exploring Rust</li>
                    </ul>
                </section>
            </div>
            <div class="mb-4 text-center md:absolute md:right-0 md:left-0 md:bottom-4 md:mb-0">
                <p class="text-gray-400">
                    Powered by
                    <a href="https://github.com/rust-dd" target="_blank" class="text-[#ffbd2e]">
                        rust-dd
                    </a> {" © "} {Utc::now().year()}
                </p>
            </div>
        </main>
    }
}
