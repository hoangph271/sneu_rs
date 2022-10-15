use crate::components::Markdown;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleProps {
    pub title: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let TitleProps { title } = props;

    html! {
        <div
            class="bg-gray-900/75 py-4 rounded-t text-center"
        >
            <Markdown markdown={title.clone()} />
        </div>
    }
}
