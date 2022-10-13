use serde::Serialize;
use yew_router::Routable;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum SneuRoutes {
    #[at("/")]
    Home,
    #[at("/user")]
    User,
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
    #[at("/uselasted/create")]
    UseLastedCreate,
    #[at("/local-library")]
    LocalLibrary,
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
