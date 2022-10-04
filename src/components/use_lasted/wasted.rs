use yew::prelude::*;

#[function_component(Wasted)]
pub fn wasted() -> Html {
    html! {
        <div
            class="bg-gray-900/75 py-4 rounded-b text-center"
        >
            { "#DONE" }
        </div>
    }
}
