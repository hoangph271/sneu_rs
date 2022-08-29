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
    #[at("/trial")]
    Trial,
}

pub fn switch(route: &SneuRoute) -> Html {
    match route {
        SneuRoute::Home => html! { <Home /> },
        SneuRoute::SignIn => html! { <SignIn /> },
        SneuRoute::Markdown { filename } => html! { <Markdown filename={filename.clone()} /> },
        SneuRoute::Gallery => html! { <Gallery /> },
        SneuRoute::Music => html! { <Music /> },
        SneuRoute::NotFound => html! { <NotFound /> },
        SneuRoute::Trial => {
            html! { <code> { "TODO: // Add something to use SneuRoute::Trial" } </code> }
        }
    }
}

pub type SneuLink = Link<SneuRoute>;
