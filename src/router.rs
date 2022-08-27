use crate::views::*;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum SneuRoute {
    #[at("/")]
    Home,
    #[at("/signin")]
    SignIn,
    #[at("/markdown/:filename")]
    Markdown { filename: String },
}

pub fn switch(route: &SneuRoute) -> Html {
    match route {
        SneuRoute::Home => html! { <Home /> },
        SneuRoute::SignIn => html! { <SignIn /> },
        SneuRoute::Markdown { filename } => html! {
            <Markdown filename={filename.clone()} />
        },
    }
}
