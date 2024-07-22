use leptos::*;
use regex::Regex;
use std::rc::Rc;

use crate::context::{CodeSetter, Exercises, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();
    let Exercises { exercise_04, .. } = expect_context::<Exercises>();
    let re = Regex::new(r"\d+\.").unwrap();
    let exercise_04 = Rc::new(re.split(exercise_04).collect::<Vec<&str>>());

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Learning References</h2>
                <p class="text-gray-400">
                    {r#"In Rust, you can create multiple immutable references to a variable, but only one mutable reference at a time. This ensures safe concurrency and avoids data races."#}
                </p>

                <div class="flex flex-col gap-1 mt-2">
                    <h3 class="text-xl font-bold">
                        Declare multiple references and one mutable reference
                    </h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[0]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_04[0]}</code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">First immutable reference</h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[1]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_04[1]}</code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Second immutable reference</h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[2]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_04[2]}</code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Mutable reference</h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[3]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 whitespace-pre-wrap bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_04[3]}</code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Print all references</h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[4]));
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
                            {String::from(exercise_04[4])}
                        </code>
                    </pre>
                </div>
                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Print all references</h3>
                    <pre
                        on:click={
                            let exercise_04 = exercise_04.clone();
                            move |_| {
                                set_code(String::from(exercise_04[5]));
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
                            {String::from(exercise_04[5])}
                        </code>
                    </pre>
                </div>
            </div>
        </div>
    }
}
