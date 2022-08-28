use crate::views::*;
use yew::{html, Html};
use yew_router::{prelude::Link, Routable};

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum SneuRoute {
    #[at("/")]
    Home,
    #[at("/signin")]
    SignIn,
    #[at("/markdown/:filename")]
    Markdown { filename: String },
    #[at("/gallery")]
    Gallery,
    #[at("/music")]
    Music,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &SneuRoute) -> Html {
    match route {
        SneuRoute::Home => html! { <Home /> },
        SneuRoute::SignIn => html! { <SignIn /> },
        SneuRoute::Markdown { filename } => html! { <Markdown filename={filename.clone()} /> },
        SneuRoute::Gallery => html! { <Gallery /> },
        SneuRoute::Music => html! { <Music /> },
        SneuRoute::NotFound => html! { <NotFound /> },
    }
}

pub type SneuLink = Link<SneuRoute>;
