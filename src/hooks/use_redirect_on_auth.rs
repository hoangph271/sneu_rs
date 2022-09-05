use crate::{providers::use_auth_context, router::sneu_routes::SneuRoutes, utils::no_op};
use web_sys::UrlSearchParams;
use yew::use_effect_with_deps;
use yew_hooks::use_location;
use yew_router::prelude::{use_history, History};
use yew_router::Routable;

pub fn use_redirect_on_auth() {
    let history = use_history().unwrap();
    let location = use_location();
    let auth_context = use_auth_context();

    use_effect_with_deps(
        {
            let auth = (*auth_context).clone();

            move |_| {
                if auth.is_authed() {
                    let redirect_url = UrlSearchParams::new_with_str(&location.search)
                        .unwrap_or_else(|e| panic!("new_with_str() failed: {e:?}"))
                        .get("redirect_url")
                        .filter(|val| !val.is_empty());

                    if let Some(redirect_url) = redirect_url {
                        history.push(SneuRoutes::recognize(&redirect_url).unwrap())
                    } else {
                        history.push(SneuRoutes::index())
                    }
                }

                no_op
            }
        },
        auth_context,
    );
}
