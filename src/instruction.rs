use leptos::*;

use crate::{context::Progress, exercises::exercise_01};

#[component]
pub fn Instruction() -> impl IntoView {
    let Progress(progress) = expect_context::<Progress>();

    view! {
        <div class="flex relative inset-0 flex-col p-4 w-full h-auto rounded-xl border border-solid md:p-8 max-w-[600px] border-[rgba(255,255,255,0.1)] bg-[#ffef5c14] shadow-[0_11px_12px_rgba(0,0,0,0.5)] md:max-w-[600px] md:h-[540px]">
            <Show when=move || progress.get() == 0>
                <exercise_01::Component></exercise_01::Component>
            </Show>
            <Show when=move || progress.get() == 1>
                <div>{"Exercise 2"}</div>
            </Show>
            <div class="flex absolute right-4 bottom-4 flex-row gap-1">
                <Show when=move || { progress.get() > 0 }>
                    <button
                        on:click=move |_| progress.update(|prev| *prev -= 1)
                        class="py-1 px-3 text-sm font-medium bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 text-[#F7E85E]"
                    >
                        Prev
                    </button>
                </Show>
                <button
                    on:click=move |_| progress.update(|prev| *prev += 1)
                    class="py-1 px-3 text-sm font-medium bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 text-[#F7E85E]"
                >
                    Next
                </button>
            </div>
        </div>
    }
}
