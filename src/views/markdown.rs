use crate::components::MarkdownViewer;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct MarkdownProps {
    pub filename: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    log::warn!("// TODO: Display markdown from url - {}", props.filename);

    html! {
        <MarkdownViewer />
    }
}
