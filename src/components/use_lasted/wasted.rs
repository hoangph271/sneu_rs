use yew::prelude::*;

#[function_component(Wasted)]
pub fn wasted() -> Html {
    html! {
        <a
            class="bg-gray-900/75 py-4 rounded-b text-center underline"
            href="https://github.com/hoangph271/sneu_rs/blob/main/src/views/use_lasted.rs"
            target="_blank"
        >
            { "@source_code" }
        </a>
    }
}
