use leptos::*;

use crate::{
    context::{Exercises, Progress},
    exercises::{exercise_00, exercise_01, exercise_02},
};

#[component]
pub fn Component() -> impl IntoView {
    let Progress(progress) = expect_context::<Progress>();
    let Exercises { count, .. } = expect_context::<Exercises>();

    view! {
        <div class="flex justify-between flex-col p-4 w-full h-auto rounded-xl border border-solid md:p-4 max-w-[600px] border-[rgba(255,255,255,0.1)] bg-[#ffef5c14] shadow-[0_11px_12px_rgba(0,0,0,0.5)] md:max-w-[600px] md:h-[540px]">
            <div class="p-4">
                <Show when=move || progress.get() == 0>
                    <exercise_01::Component />
                </Show>
                <Show when=move || progress.get() == 1>
                    <exercise_02::Component />
                </Show>
                <Show when=move || progress.get() == count>
                    <exercise_00::Component />
                </Show>
            </div>
            <div class="flex items-end justify-end mt-4 flex-row gap-1">
                <Show when=move || { progress.get() > 0 }>
                    <button
                        on:click=move |_| progress.update(|prev| *prev -= 1)
                        class="py-1 px-3 text-sm font-medium bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 text-[#F7E85E]"
                    >
                        Prev
                    </button>
                </Show>
                <Show when=move || { progress.get() <= count }>
                    <button
                        on:click=move |_| {
                            if progress.get() < count {
                                progress.update(|prev| *prev += 1);
                            } else {
                                progress.set(0);
                            }
                        }
                        class="py-1 px-3 text-sm font-medium bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 text-[#F7E85E]"
                    >
                        Next
                    </button>
                </Show>
            </div>
        </div>
    }
}
