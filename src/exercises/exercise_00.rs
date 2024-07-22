use leptos::*;

#[component]
pub fn Component() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 w-full text-white rounded-lg">
            <div class="flex flex-col gap-2">
                <div class="text-3xl font-bold text-white">Congratulations!</div>
                <p class="text-gray-400">
                    {"Congratulations, you've started your Rust journey! Here are some useful links to help you along the way:"}
                </p>
                <ul class="list-disc list-inside text-gray-400">
                    <li>
                        <a href="https://www.rust-lang.org/" class="text-[#ffbd2e]">
                            Official Rust Website
                        </a>
                    </li>
                    <li>
                        <a href="https://doc.rust-lang.org/book/" class="text-[#ffbd2e]">
                            The Rust Programming Language (The Book)
                        </a>
                    </li>
                    <li>
                        <a href="https://docs.rs/" class="text-[#ffbd2e]">
                            Rust Documentation
                        </a>
                    </li>
                    <li>
                        <a href="https://crates.io/" class="text-[#ffbd2e]">
                            Crates.io (Rust Package Registry)
                        </a>
                    </li>
                    <li>
                        <a href="https://users.rust-lang.org/" class="text-[#ffbd2e]">
                            Rust Users Forum
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://stackoverflow.com/questions/tagged/rust"
                            class="text-[#ffbd2e]"
                        >
                            Rust on Stack Overflow
                        </a>
                    </li>
                    <li>
                        <a href="https://rust-lang.github.io/async-book/" class="text-[#ffbd2e]">
                            Asynchronous Programming in Rust
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
