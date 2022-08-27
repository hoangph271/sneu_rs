use crate::views::*;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Routable)]
pub enum SneuRoute {
    #[at("/")]
    Home,
    #[at("/signin")]
    SignIn,
}

pub fn switch(route: &SneuRoute) -> Html {
    match route {
        SneuRoute::Home => html! { <Home /> },
        SneuRoute::SignIn => html! { <SignIn /> },
    }
}
