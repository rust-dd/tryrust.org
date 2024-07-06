use crate::{instruction::Instruction, terminal::Terminal};
use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tryrust.css"/>
        <main class="flex flex-row items-center justify-center w-screen h-screen bg-[#1e1e1e]">
            <Terminal />
            <Instruction />
        </main>
    }
}
