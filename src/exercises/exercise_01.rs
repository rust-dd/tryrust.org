use leptos::*;

use crate::terminal::{CodeSetter, InputRef};

#[component]
pub fn Component() -> impl IntoView {
    let CodeSetter(set_code) = expect_context::<CodeSetter>();
    let InputRef(_ref) = expect_context::<InputRef>();

    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <div class="text-3xl font-bold text-white">Why Rust?</div>
                <p class="text-gray-400">
                    {"Rust is fast and memory-efficient, with no runtime or garbage collector, making it ideal for performance-critical services and embedded devices. Its type system and ownership model ensure memory and thread safety, catching many bugs at compile-time. Rust offers excellent documentation, a helpful compiler, and superior tooling, including a package manager, build tool, multi-editor support, auto-completion, type inspections, and an auto-formatter."}
                </p>
            </div>
            <div class="flex flex-col gap-2">
                <h2 class="text-3xl font-bold">Hello, World!</h2>
                <p class="text-gray-400">
                    {r#"Create a simple Rust program that prints "Hello, world!" to the console using the"#}
                    <code class="p-1 bg-gray-700 rounded">println!</code>macro.
                </p>
                <pre
                    on:click=move |_| {
                        set_code(r#"println!("Hello, world!");"#.to_string());
                        _ref().get().expect("input_ref should be loaded by now").focus().unwrap();
                    }
                    class="p-4 mt-2 bg-black bg-opacity-30 rounded-lg transition duration-500 cursor-pointer hover:bg-black hover:bg-opacity-50"
                >
                    <code class="font-mono text-sm text-yellow-500">
                        {r#"println!("Hello, world!");"#}
                    </code>
                </pre>
            </div>
        </div>
    }
}
