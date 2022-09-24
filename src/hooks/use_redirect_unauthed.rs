use crate::{
    providers::{use_auth_context, AuthMessage},
    router::{SignInQuery, SneuRoutes},
    utils::no_op,
};

pub use components::*;
use yew::use_effect_with_deps;
use yew_hooks::use_location;
use yew_router::prelude::{use_history, History};

fn use_redirect_unauthed() -> AuthMessage {
    let history = use_history().unwrap();
    let location = use_location();
    let auth_context = use_auth_context();

    use_effect_with_deps(
        {
            let auth = (*auth_context).clone();

            move |_| {
                if !auth.is_authed() {
                    let pathname = location.pathname.clone();

                    let redirect_url = if pathname.eq("/") {
                        None
                    } else {
                        Some(format!("{}{}", pathname, location.search.clone()))
                    };

                    history
                        .replace_with_query(SneuRoutes::SignIn, SignInQuery { redirect_url })
                        .unwrap();
                }

                no_op
            }
        },
        (*auth_context).clone(),
    );

    (*auth_context).clone()
}

mod components {
    use super::use_redirect_unauthed;
    use yew::prelude::*;

    pub fn use_with_auth_required(render: impl Fn() -> Html) -> Html {
        let auth = use_redirect_unauthed();

        html! {
            if auth.is_authed() {
                { render() }
            }
        }
    }
}
