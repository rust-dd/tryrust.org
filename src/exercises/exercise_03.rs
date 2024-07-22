use leptos::*;
use regex::Regex;
use std::rc::Rc;

use crate::context::{CodeSetter, Exercises, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();
    let Exercises { exercise_03, .. } = expect_context::<Exercises>();
    let re = Regex::new(r"\d+\.").unwrap();
    let exercise_03 = Rc::new(re.split(exercise_03).collect::<Vec<&str>>());

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Learning Mutable Variables</h2>
                <p class="text-gray-400">
                    {r#"In Rust, variables are immutable by default. To modify a variable's value, it must be declared as mutable using the `mut` keyword."#}
                </p>

                <div class="flex flex-col gap-1 mt-2">
                    <h3 class="text-xl font-bold">Declare a mutable variable</h3>
                    <pre
                        on:click={
                            let exercise_03 = exercise_03.clone();
                            move |_| {
                                set_code(String::from(exercise_03[0]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 whitespace-pre-wrap"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_03[0]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Modify the mutable variable</h3>
                    <pre
                        on:click={
                            let exercise_03 = exercise_03.clone();
                            move |_| {
                                set_code(String::from(exercise_03[1]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 whitespace-pre-wrap"
                    >
                        <code class="font-mono text-sm text-yellow-500">{exercise_03[1]}</code>
                    </pre>
                </div>

                <div class="flex flex-col gap-1">
                    <h3 class="text-xl font-bold">Print the modified value</h3>
                    <pre
                        on:click={
                            let exercise_03 = exercise_03.clone();
                            move |_| {
                                set_code(String::from(exercise_03[2]));
                                _ref()
                                    .get()
                                    .expect("input_ref should be loaded by now")
                                    .focus()
                                    .unwrap();
                            }
                        }
                        class="p-4 bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50 whitespace-pre-wrap"
                    >
                        <code class="font-mono text-sm text-left text-yellow-500">
                            {move || String::from(exercise_03[2])}
                        </code>
                    </pre>
                </div>
            </div>
        </div>
    }
}
