use leptos::*;

use crate::terminal::{CodeSetter, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();

    view! {
        <div class="bg-[#2b2b2b] p-4 text-white rounded-lg w-full max-w-md">
          <h2 class="text-2xl font-bold">Rust: Hello, World!</h2>
          <p class="text-muted-foreground">
            Create a simple Rust program that prints "Hello, world!" to the console using the <code>println!</code> macro.
          </p>
          <pre
            on:click={move |_| {
                set_code(r#"println!("Hello, world!");"#.to_string());
                _ref().get().expect("input_ref should be loaded by now").focus().unwrap();
            }}
            class="mt-2 bg-slate-200 p-2 rounded-lg cursor-pointer">
            <code class="text-sm font-mono text-red-500">{r#"println!("Hello, world!");"#}</code>
          </pre>
        </div>
    }
}
