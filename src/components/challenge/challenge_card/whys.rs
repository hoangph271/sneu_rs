use crate::components::Markdown;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct WhysProps {
    #[prop_or_default]
    pub class: Classes,
    pub text: String,
}

#[function_component(Whys)]
pub fn whys(props: &WhysProps) -> Html {
    let WhysProps { text, class } = props;

    html! {
        <Markdown
            class={classes!(format!("bg-slate-700/75 p-2 overflow-auto {}", class.to_string()))}
            markdown={text.clone()}
        />
    }
}
