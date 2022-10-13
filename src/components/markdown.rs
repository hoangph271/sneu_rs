use super::unsafe_html::UnsafeHtml;
use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MarkdownProps {
    #[prop_or_default]
    pub class: Classes,
    pub markdown: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let MarkdownProps { markdown, class } = props;

    html! {
        <UnsafeHtml
            class={class.clone()}
            html={ parse_markdown(markdown) }
            tag="div"
        />
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
