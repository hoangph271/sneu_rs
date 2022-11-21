use crate::{
    providers::use_auth_context,
    router::{SignInQuery, SneuRoutes},
    utils::no_op,
};

pub use components::*;
use yew::use_effect_with_deps;
use yew_hooks::use_location;
use yew_router::prelude::History;

use super::use_history;

pub fn use_redirect_unauthed() -> bool {
    let history = use_history();
    let location = use_location();
    let auth_context = use_auth_context();

    use_effect_with_deps(
        {
            let auth_context = auth_context.clone();

            move |_| {
                match auth_context {
                    Some(x) if x.is_authed() => {}
                    _ => {
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
                }

                no_op
            }
        },
        auth_context.clone(),
    );

    match auth_context {
        Some(auth) => auth.is_authed(),
        None => false,
    }
}

pub mod components {
    use super::use_redirect_unauthed;
    use yew::prelude::*;

    pub fn use_with_auth_required(render: impl Fn() -> Html) -> Html {
        let is_auth = use_redirect_unauthed();

        html! {
            if is_auth {
                { render() }
            }
        }
    }
}
