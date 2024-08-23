use leptos::*;

use crate::{
    context::{Exercises, Progress},
    exercises::{exercise_00, exercise_01, exercise_02, exercise_03, exercise_04, exercise_05, exercise_06},
};

#[component]
pub fn Component() -> impl IntoView {
    let Progress(progress) = expect_context::<Progress>();
    let Exercises { count, .. } = expect_context::<Exercises>();

    view! {
        <div class="flex flex-col justify-between p-4 w-full h-auto rounded-xl border border-solid md:p-4 max-w-[600px] border-[rgba(255,255,255,0.1)] bg-[#ffef5c14] shadow-[0_11px_12px_rgba(0,0,0,0.5)] md:max-w-[600px] md:h-[540px]">
            <div class="overflow-auto p-4 scrollbar-thin scrollbar-track-transparent scrollbar-thumb-scrollbar-thumb">
                <Show when=move || progress.get() == 0>
                    <exercise_01::Component />
                </Show>
                <Show when=move || progress.get() == 1>
                    <exercise_02::Component />
                </Show>
                <Show when=move || progress.get() == 2>
                    <exercise_03::Component />
                </Show>
                <Show when=move || progress.get() == 3>
                    <exercise_04::Component />
                </Show>
                <Show when=move || progress.get() == 4>
                    <exercise_05::Component />
                </Show>
                <Show when=move || progress.get() == 5>
                    <exercise_06::Component />
                </Show>
                <Show when=move || progress.get() == count>
                    <exercise_00::Component />
                </Show>
            </div>
            <div class="flex flex-row gap-1 justify-end items-end mt-4">
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
