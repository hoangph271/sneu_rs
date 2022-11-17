use crate::components::Markdown;
use crate::router::{SneuLink, SneuRoutes};
use js_sys::encode_uri_component;
use yew::prelude::*;

#[derive(PartialEq, Properties, Eq)]
pub struct TitleProps {
    pub id: String,
    pub title: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let TitleProps { title, id } = props;

    html! {
        <SneuLink
            to={SneuRoutes::UseLastedEdit { id: encode_uri_component(id).into() }}
            classes="bg-gray-900/75 py-4 rounded-t text-center hover:underline"
        >
            <Markdown markdown={title.clone()} />
        </SneuLink>
    }
}
