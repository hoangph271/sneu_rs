use crate::views::*;
use sneu_routes::*;
use yew::{html, Html};
pub use yew_router::prelude::Link;

pub mod sneu_routes {
    use serde::Serialize;
    use yew_router::Routable;

    #[derive(Debug, Clone, PartialEq, Eq, Routable)]
    pub enum SneuRoutes {
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
        #[at("/sneu-player")]
        SneuPlayer,
        #[not_found]
        #[at("/404")]
        NotFound,
        #[at("/trial")]
        Trial,
    }

    impl SneuRoutes {
        pub fn index() -> Self {
            if cfg!(feature = "sneu_tauri") {
                SneuRoutes::SneuPlayer
            } else {
                SneuRoutes::Home
            }
        }
    }

    #[derive(Serialize)]
    pub struct SignInQuery {
        pub redirect_url: Option<String>,
    }
}

pub fn switch(route: &SneuRoutes) -> Html {
    match route {
        SneuRoutes::Home => html! { <Home /> },
        SneuRoutes::SignIn => html! { <SignIn /> },
        SneuRoutes::Markdown { filename } => html! { <Markdown filename={filename.clone()} /> },
        SneuRoutes::Gallery => html! { <Gallery /> },
        SneuRoutes::Music => html! { <Music /> },
        SneuRoutes::SneuPlayer => html! { <SneuPlayer /> },
        SneuRoutes::NotFound => html! { <NotFound /> },
        SneuRoutes::Trial => html! { <Trial /> },
    }
}

pub type SneuLink = Link<SneuRoutes>;
