use crate::{
    components::{with_loader, MarkdownViewer},
    utils::no_op,
};
use gloo_net::http::Request;
use urlencoding::decode;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;

#[derive(PartialEq, Eq, Properties)]
pub struct MarkdownProps {
    pub url: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let markdown = use_state_eq(|| Option::<String>::None);
    let MarkdownProps { url } = props;

    use_effect_once({
        let url = url.clone();
        let markdown = markdown.clone();

        move || {
            spawn_local(async move {
                let url = decode(&url).unwrap();
                let content = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                markdown.set(Some(content));
            });

            no_op
        }
    });

    with_loader((*markdown).clone(), |markdown| {
        html! {
            <MarkdownViewer markdown={markdown} />
        }
    })
}
