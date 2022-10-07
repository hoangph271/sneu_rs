use super::SneuRoutes;
use crate::views::*;
use yew::{html, Html};

pub fn switch(route: &SneuRoutes) -> Html {
    match route {
        SneuRoutes::Home => html! { <Home /> },
        SneuRoutes::User => html! { <User /> },
        SneuRoutes::SignIn => html! { <SignIn /> },
        SneuRoutes::Markdown { url } => html! { <Markdown url={url.clone()} /> },
        SneuRoutes::Gallery => html! { <Gallery /> },
        SneuRoutes::Music => html! { <Music /> },
        SneuRoutes::SneuPlayer => html! { <SneuPlayer /> },
        SneuRoutes::NotFound => html! { <NotFound /> },
        SneuRoutes::Trial => html! { <Trial /> },
        SneuRoutes::UseLasted => html! { <UseLasted /> },
        SneuRoutes::LocalLibrary => html! { <LocalLibrary /> },
    }
}
