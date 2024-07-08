use leptos::*;

#[component]
pub fn Instruction() -> impl IntoView {
    view! {
        <div class="bg-[#ffef5c] max-w-[600px] w-full h-[320px] p-8 rounded-xl md:max-w-[600px] md:h-[540px]">
            <div class="text-3xl font-bold">Got 5 minutes?</div>
        </div>
    }
}
