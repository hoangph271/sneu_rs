use crate::{
    providers::{use_auth_context, AuthMessage},
    router::SneuRoute,
    utils::no_op,
};

use yew::use_effect_with_deps;
use yew_router::prelude::{use_history, History};

pub fn use_redirect_on_auth() {
    let history = use_history().unwrap();
    let auth_context = use_auth_context();

    use_effect_with_deps(
        {
            let auth = (*auth_context).clone();

            move |_| {
                if let AuthMessage::Authed(_) = auth {
                    history.push(SneuRoute::Home)
                }

                no_op
            }
        },
        auth_context,
    );
}
