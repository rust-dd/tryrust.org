use leptos::*;
use regex::Regex;
use std::rc::Rc;

use crate::context::{CodeSetter, Exercises, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();
    let Exercises { exercise_06, .. } = expect_context::<Exercises>();
    let re = Regex::new(r"\d+\.").unwrap();
    let exercise_06 = Rc::new(re.split(exercise_06).collect::<Vec<&str>>());

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Learning by Functions</h2>
                <p class="text-gray-400">
                    {r#"In this exercise, we'll define and use functions in Rust."#}
                </p>

                <div class="flex flex-col gap-1 mt-2">
                    <h3 class="text-xl font-bold">Step 1: Define a Function without Return</h3>
                    <pre
                        on:click={
                            let exercise_06 = exercise_06.clone();
                            move |_| {
                                set_code(String::from(exercise_06[0]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_06[0]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 2: Define a Function with Return</h3>
                    <pre
                        on:click={
                            let exercise_06 = exercise_06.clone();
                            move |_| {
                                set_code(String::from(exercise_06[1]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_06[1]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 3: Call Your Greet Functions</h3>
                    <pre
                        on:click={
                            let exercise_06 = exercise_06.clone();
                            move |_| {
                                set_code(String::from(exercise_06[2]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_06[2]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 4: Call Your Dice Functions</h3>
                    <pre
                        on:click={
                            let exercise_06 = exercise_06.clone();
                            move |_| {
                                set_code(String::from(exercise_06[3]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_06[3]}</code>
                    </pre>
                </div>
            </div>
        </div>
    }
}
