use crate::components::*;
use markdown::Markdown;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct MarkdownViewerProps {
    pub markdown: String,
}

#[function_component(MarkdownViewer)]
pub fn markdown_viewer(props: &MarkdownViewerProps) -> Html {
    let is_in_monospace = use_state_eq(|| false);
    let MarkdownViewerProps { markdown } = props;
    let font_style = if *is_in_monospace {
        "Serif"
    } else {
        "Monospace"
    };

    html! {
        <div>
            <PillButton
                onclick={
                    Callback::from(move |_| is_in_monospace.set(!*is_in_monospace))
                }
            >
                { font_style }
            </PillButton>
            <div>
                <Markdown markdown={markdown.clone()} />
            </div>
        </div>
    }
}
