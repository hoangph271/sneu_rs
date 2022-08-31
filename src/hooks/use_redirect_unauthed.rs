use crate::{providers::use_auth_context, router::SneuRoute, utils::no_op};

use yew::use_effect_with_deps;
use yew_router::prelude::{use_history, History};

pub fn use_redirect_unauthed() {
    let history = use_history().unwrap();
    let auth_context = use_auth_context();

    use_effect_with_deps(
        {
            let auth = (*auth_context).clone();

            move |_| {
                if !auth.is_authed() {
                    history.push(SneuRoute::SignIn)
                }

                no_op
            }
        },
        auth_context,
    );
}
