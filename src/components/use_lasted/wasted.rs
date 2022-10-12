use crate::components::Markdown;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WastedProps {
    pub note: String,
}

#[function_component(Wasted)]
pub fn wasted(props: &WastedProps) -> Html {
    let WastedProps { note } = props;

    html! {
        <div
            class="bg-gray-900/75 py-4 rounded-b text-center"
        >
            <Markdown markdown={note.clone()} />
        </div>
    }
}
