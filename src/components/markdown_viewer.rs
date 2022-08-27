use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

use super::unsafe_html::UnsafeHtml;

pub enum MarkdownViewerMessage {
    ToggleMonospace,
}

pub struct MarkdownViewer {
    is_in_monospace: bool,
    markdown: String,
}

impl Component for MarkdownViewer {
    type Message = MarkdownViewerMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_in_monospace: true,
            markdown: String::from_utf8(include_bytes!("../../LICENSE.md").to_vec())
                .unwrap_or_else(|e| panic!("{e}")),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MarkdownViewerMessage::ToggleMonospace => {
                self.is_in_monospace = !self.is_in_monospace;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let font_style = if self.is_in_monospace {
            "Serif"
        } else {
            "Monospace"
        };
        let markdown_root_tag = if self.is_in_monospace { "code" } else { "div" };

        html! {
            <div>
                <button onclick={link.callback(|_| MarkdownViewerMessage::ToggleMonospace)}>
                    { font_style }
                </button>
                <UnsafeHtml
                    html={ parse_markdown(&self.markdown) }
                    tag={ markdown_root_tag }
                />
            </div>
        }
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
