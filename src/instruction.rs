use leptos::*;

use crate::exercises::exercise_01;

#[component]
pub fn Instruction() -> impl IntoView {
    view! {
        <div class="max-w-[600px] w-full h-auto border border-solid border-[rgba(255,255,255,0.1)] bg-[#ffef5c14] shadow-[0_11px_12px_rgba(0,0,0,0.5)] inset-0 flex flex-col p-4 md:p-8 rounded-xl md:max-w-[600px] md:h-[540px]">
            <exercise_01::Component />
        </div>
    }
}
