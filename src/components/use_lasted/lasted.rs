use super::utils::use_lasted;
use yew::prelude::*;

#[function_component(Lasted)]
pub fn lasted() -> Html {
    let (lasted, progress) = use_lasted();

    html! {
        <div class="bg-gray-900/75 py-4 text-center rounded-t">
            <span>{ "You lasted " }</span>
            <span style="font-style: italic">
                { format!("{lasted} [{progress}%]...!") }
            </span>
        </div>
    }
}
