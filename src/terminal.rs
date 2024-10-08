use codee::string::FromToStringCodec;
use regex::Regex;
use std::{collections::BTreeMap, sync::Arc};

use html::Input;
use leptos::*;
use leptos_use::{storage::use_session_storage, use_event_listener};

use crate::{
    context::{CodeSetter, Exercises, InputRef, Progress},
    server::compile::compile,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum TerminalEvent {
    Code,
    Error,
    Success,
}

#[component]
pub fn Component() -> impl IntoView {
    let exercises = Arc::new(expect_context::<Exercises>());
    let data = create_rw_signal(BTreeMap::<(usize, TerminalEvent), String>::new());
    let (code, set_code) = create_signal(String::new());
    provide_context(CodeSetter(set_code));
    let (r#ref, ..) = create_signal(create_node_ref::<Input>());
    let _ref = r#ref();
    provide_context(InputRef(r#ref));
    let (session_id, ..) = use_session_storage::<String, FromToStringCodec>("session_id");
    let Progress(progress) = expect_context::<Progress>();
    let compile_code = create_action(
        move |(session_id, code, exercises): &(String, String, Arc<Exercises>)| {
            let session_id = session_id.clone();
            let code = code.clone();
            let exercise_01 = exercises.exercise_01;
            let exercise_02 = exercises.exercise_02;
            let re = Regex::new(r"\d+\.").unwrap();
            let exercise_02 = re.split(exercise_02).collect::<Vec<&str>>();
            let exercise_02 = exercise_02[2];
            let exercise_03 = exercises.exercise_03;
            let exercise_03 = re.split(exercise_03).collect::<Vec<&str>>();
            let exercise_03 = exercise_03[2];
            let exercise_04 = exercises.exercise_04;
            let exercise_04 = re.split(exercise_04).collect::<Vec<&str>>();
            let exercise_04 = exercise_04[5];
            let exercise_05 = exercises.exercise_05;
            let exercise_05 = re.split(exercise_05).collect::<Vec<&str>>();
            let exercise_05 = exercise_05[3];
            let exercise_06 = exercises.exercise_06;
            let exercise_06 = re.split(exercise_06).collect::<Vec<&str>>();
            let exercise_06 = exercise_06[3];
            let exercise_07 = exercises.exercise_07;
            let exercise_07 = re.split(exercise_07).collect::<Vec<&str>>();
            let exercise_07 = exercise_07[5];


            data.update(|prev| {
                prev.insert((prev.len(), TerminalEvent::Code), code.clone());
            });

            set_code(Default::default());

            async move {
                let response = compile(session_id.clone(), code.clone())
                    .await
                    .expect("Failed to compile");

                if !response.is_empty() {
                    let is_error = response.contains("error");
                    data.update(|prev| {
                        let key = is_error
                            .then_some(TerminalEvent::Error)
                            .unwrap_or(TerminalEvent::Success);
                        prev.insert((prev.len(), key), response);
                    });

                    if !is_error {
                        if code == exercise_01 && progress.get() == 0 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 1 && code == *exercise_02 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 2 && code == *exercise_03 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 3 && code == *exercise_04 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 4 && code == *exercise_05 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 5 && code == *exercise_06 {
                            progress.update(|prev| *prev += 1);
                        } else if progress.get() == 6 && code == *exercise_07 {
                            progress.update(|prev| *prev += 1);
                        }
                    }
                }

                r#ref()
                    .get()
                    .expect("input_ref should be loaded by now")
                    .scroll_into_view();
            }
        },
    );

    let _ = use_event_listener(r#ref(), ev::keydown, move |e| {
        if compile_code.pending()() || code().is_empty() {
            return;
        }

        if e.key() == "Enter" {
            compile_code.dispatch((session_id(), code(), exercises.clone()));
        }
    });

    view! {
        <div class="flex relative flex-col w-full rounded-xl md:rounded-r-none md:rounded-l-xl max-w-[600px] h-[400px] bg-[#2b2b2b]">
            <div class="absolute -right-0 -top-8 animate-move">
                <img src="crab.png" class="w-12 animate-wiggle" />
            </div>
            <div class="flex justify-between items-center py-2 px-4 rounded-t-xl border-b md:rounded-tr-none md:rounded-tl-xl bg-[#3c3c3c] border-[#4d4d4d]">
                <div class="flex gap-2 items-center">
                    <div class="w-3 h-3 rounded-full bg-[#ff5f56]"></div>
                    <div class="w-3 h-3 rounded-full bg-[#ffbd2e]"></div>
                    <div class="w-3 h-3 rounded-full bg-[#27c93f]"></div>
                </div>
                <div class="text-sm text-[#9e9e9e]">Terminal</div>
                <div />
            </div>
            <div class="overflow-auto flex-1 px-4 pt-4 mb-8 font-mono text-sm leading-relaxed scrollbar-thin scrollbar-track-transparent scrollbar-thumb-scrollbar-thumb text-[#c6c6c6]">
                <For
                    each=move || data.get().into_iter()
                    key=|((idx, _), _)| *idx
                    children=|((_, r#type), code)| {
                        match r#type {
                            TerminalEvent::Error => {
                                view! {
                                    <div class="flex gap-4 py-1 items">
                                        <span class="text-[#ff5f56]">></span>
                                        <span>{code}</span>
                                    </div>
                                }
                            }
                            TerminalEvent::Success => {
                                view! {
                                    <div class="flex gap-4 py-1 items">
                                        <span class="text-[#27c93f]">></span>
                                        <span>{code}</span>
                                    </div>
                                }
                            }
                            TerminalEvent::Code => {
                                view! {
                                    <div class="flex gap-4 py-1 items">
                                        <span class="text-[#ffef5c]">$</span>
                                        <span>{code}</span>
                                    </div>
                                }
                            }
                        }
                    }
                />
                <div class="flex gap-2 items-center">
                    <span class="text-[#ffef5c]">$</span>
                    <input
                        _ref=_ref
                        name="code"
                        r#type="text"
                        autocomplete="off"
                        class="py-1 px-2 w-full bg-transparent border-none outline-none text-[#c6c6c6] caret-[#ffef5c]"
                        placeholder="Type a command..."
                        prop:value=move || code()
                        autofocus=true
                        on:input=move |e| set_code(event_target_value(&e))
                    />
                </div>
            </div>
            <div class="absolute right-4 bottom-4">
                <Show
                    when=move || !compile_code.pending()()
                    fallback=move || {
                        view! {
                            <div class="flex flex-row gap-2 justify-center items-center">
                                <img
                                    src="rust_color.png"
                                    width=16
                                    height=16
                                    class="animate-spin-fast"
                                />
                                <span class="text-sm text-white">Compiling...</span>
                            </div>
                        }
                    }
                >
                    <div />
                </Show>
            </div>
        </div>
    }
}
