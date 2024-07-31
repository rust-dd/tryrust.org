use leptos::*;
use regex::Regex;
use std::rc::Rc;

use crate::context::{CodeSetter, Exercises, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();
    let Exercises { exercise_05, .. } = expect_context::<Exercises>();
    let re = Regex::new(r"\d+\.").unwrap();
    let exercise_05 = Rc::new(re.split(exercise_05).collect::<Vec<&str>>());

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Learning by Structs and Operator Overloading</h2>
                <p class="text-gray-400">
                    {r#"In this exercise, we will define a Vec struct with x and y fields and implement the Add operator for it."#}
                </p>

                <div class="flex flex-col gap-1 mt-2">
                    <h3 class="text-xl font-bold">Step 1: Define the Vec struct</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[0]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_05[0]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 2: Implement Add for Vec</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[1]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_05[1]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 3: Declare v1</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[2]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_05[2]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 4: Declare v2</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[3]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_05[3]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 5: Add v1 and v2</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[4]));
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
                            {exercise_05[4]}
                        </code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Step 5: Print v1, v2, and v3</h3>
                    <pre
                        on:click={
                            let exercise_05 = exercise_05.clone();
                            move |_| {
                                set_code(String::from(exercise_05[5]));
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
                            {move || String::from(exercise_05[5])}
                        </code>
                    </pre>
                </div>
            </div>
        </div>
    }
}
