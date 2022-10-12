use crate::components::Markdown;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WhysProps {
    pub why: String,
}

#[function_component(Whys)]
pub fn whys(props: &WhysProps) -> Html {
    let WhysProps { why } = props;

    html! {
        <ul class="bg-slate-700/75 p-2">
            <Markdown markdown={why.clone()} />
        </ul>
    }
}
