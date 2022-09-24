use crate::views::*;
use yew::{html, Html};
use yew_router::prelude::Link;
mod routes;
pub use routes::*;

pub fn switch(route: &SneuRoutes) -> Html {
    match route {
        SneuRoutes::Home => html! { <Home /> },
        SneuRoutes::SignIn => html! { <SignIn /> },
        SneuRoutes::Markdown { url } => html! { <Markdown url={url.clone()} /> },
        SneuRoutes::Gallery => html! { <Gallery /> },
        SneuRoutes::Music => html! { <Music /> },
        SneuRoutes::SneuPlayer => html! { <SneuPlayer /> },
        SneuRoutes::NotFound => html! { <NotFound /> },
        SneuRoutes::Trial => html! { <Trial /> },
        SneuRoutes::UseLasted => html! { <UseLasted /> },
        #[cfg(sneu_tauri)]
        SneuRoutes::LocalLibrary => html! { <LocalLibrary /> },
    }
}

pub type SneuLink = Link<SneuRoutes>;
