use yew::prelude::*;

use pulldown_cmark::{html, Options, Parser};

enum Msg {
    ToggleMonospace,
}

struct MarkdownViewer {
    is_in_monospace: bool,
    markdown: String,
}

impl Component for MarkdownViewer {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_in_monospace: true,
            markdown: String::from_utf8(include_bytes!("../LICENSE.md").to_vec())
                .unwrap_or_else(|e| panic!("{e}")),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMonospace => {
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
                <button onclick={link.callback(|_| Msg::ToggleMonospace)}>
                    { font_style }
                </button>
                <SafeHtml
                    html={ parse_markdown(&self.markdown) }
                    tag={ markdown_root_tag }
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<MarkdownViewer>();
}

fn parse_markdown(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[derive(Properties, PartialEq)]
pub struct SafeHtmlProps {
    pub html: String,
    pub tag: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &SafeHtmlProps) -> Html {
    let code = gloo_utils::document()
        .create_element(&props.tag)
        .unwrap_or_else(|e| panic!("create_element() [{:?}] failed: {e:?}", &props.tag));

    code.set_inner_html(&props.html.clone());

    Html::VRef(code.into())
}
