use crate::components::*;
use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

use super::unsafe_html::UnsafeHtml;

#[derive(PartialEq, Properties)]
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
                <UnsafeHtml
                    html={ parse_markdown(&markdown) }
                    tag="div"
                />
            </div>
        </div>
    }
}

fn parse_markdown(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}
