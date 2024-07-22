use leptos::*;
use regex::Regex;
use std::rc::Rc;

use crate::context::{CodeSetter, Exercises, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();
    let Exercises { exercise_02, .. } = expect_context::<Exercises>();
    let re = Regex::new(r"\d+\.").unwrap();
    let exercise_02 = Rc::new(re.split(exercise_02).collect::<Vec<&str>>());

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Learning By Numbers</h2>
                <p class="text-gray-400">
                    {r#"Rust offers several number types, such as usize, isize, u8-u128, i8-i128, f32, and f64."#}
                </p>

                <div class="flex flex-col gap-1 mt-2">
                    <h3 class="text-xl font-bold">Declare the first variable</h3>
                    <pre
                        on:click={
                            let exercise_02 = exercise_02.clone();
                            move |_| {
                                set_code(String::from(exercise_02[0]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_02[0]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Declare the second variable</h3>
                    <pre
                        on:click={
                            let exercise_02 = exercise_02.clone();
                            move |_| {
                                set_code(String::from(exercise_02[1]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_02[1]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Print the sum of the variables</h3>
                    <pre
                        on:click={
                            let exercise_02 = exercise_02.clone();
                            move |_| {
                                set_code(String::from(exercise_02[2]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-left text-yellow-500">
                            {move || String::from(exercise_02[2])}
                        </code>
                    </pre>
                </div>
            </div>
        </div>
    }
}
