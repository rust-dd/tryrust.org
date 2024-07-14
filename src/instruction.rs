use leptos::*;

use crate::exercises::exercise_01;

#[component]
pub fn Instruction() -> impl IntoView {
    view! {
        <div class="flex inset-0 flex-col p-4 w-full h-auto rounded-xl border border-solid md:p-8 max-w-[600px] border-[rgba(255,255,255,0.1)] bg-[#ffef5c14] shadow-[0_11px_12px_rgba(0,0,0,0.5)] md:max-w-[600px] md:h-[540px]">
            <exercise_01::Component></exercise_01::Component>
        </div>
    }
}
