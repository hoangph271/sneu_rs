mod profile;
mod use_profile;

use crate::hooks::use_redirect_unauthed;
use crate::providers::{use_auth_context, AuthAction, AuthMessage};
use profile::*;
use use_profile::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn index() -> Html {
    let auth_context = use_auth_context();
    let profile = use_profile();

    use_redirect_unauthed();

    match profile {
        Some(profile) => html! {
            <Profile
                profile={profile}
                on_sign_out={Callback::from(move |_| {
                    AuthMessage::remove_locally();
                    auth_context.dispatch(AuthAction::SignOut);
                })}
            />
        },
        _ => html! {},
    }
}
