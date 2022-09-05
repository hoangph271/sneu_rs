mod profile;
mod use_profile;

use crate::{
    components::with_loader,
    hooks::use_with_auth_required,
    providers::{use_auth_context, AuthAction, AuthMessage},
};
use profile::*;
use use_profile::*;
use yew::prelude::*;

#[function_component(UserProfile)]
fn user_profile() -> Html {
    let auth_context = use_auth_context();
    let profile = use_profile();

    with_loader(profile, |profile| {
        html! {
            <Profile
                profile={profile}
                on_sign_out={Callback::from({
                    let auth_context = auth_context.clone();

                    move |_| {
                        AuthMessage::remove_locally();
                        auth_context.dispatch(AuthAction::SignOut);
                    }
                })}
            />
        }
    })
}

#[function_component(Home)]
pub fn index() -> Html {
    use_with_auth_required(|| {
        html! { <UserProfile /> }
    })
}
