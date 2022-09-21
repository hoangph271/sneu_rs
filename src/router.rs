use crate::views::*;
use sneu_routes::*;
use yew::{html, Html};
use yew_router::prelude::Link;

pub mod sneu_routes {
    use serde::Serialize;
    use yew_router::Routable;

    #[derive(Debug, Clone, PartialEq, Eq, Routable)]
    pub enum SneuRoutes {
        #[at("/")]
        Home,
        #[at("/signin")]
        SignIn,
        #[at("/markdown/:url")]
        Markdown { url: String },
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
        #[at("/uselasted")]
        UseLasted,
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
        SneuRoutes::Home => html! { <UseLasted /> },
        SneuRoutes::SignIn => html! { <SignIn /> },
        SneuRoutes::Markdown { url } => html! { <Markdown url={url.clone()} /> },
        SneuRoutes::Gallery => html! { <Gallery /> },
        SneuRoutes::Music => html! { <Music /> },
        SneuRoutes::SneuPlayer => html! { <SneuPlayer /> },
        SneuRoutes::NotFound => html! { <NotFound /> },
        SneuRoutes::Trial => html! { <Trial /> },
        SneuRoutes::UseLasted => html! { <UseLasted /> },
    }
}

pub type SneuLink = Link<SneuRoutes>;
