use crate::components::Markdown;
use crate::router::{SneuLink, SneuRoutes};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleProps {
    pub id: String,
    pub title: String,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let TitleProps { title, id } = props;

    html! {
        <SneuLink
            to={SneuRoutes::UseLastedEdit { id: id.to_owned() }}
            classes="bg-gray-900/75 py-4 rounded-t text-center hover:underline"
        >
            <Markdown markdown={title.clone()} />
        </SneuLink>
    }
}
