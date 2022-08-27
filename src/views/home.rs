use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::components::{BulmaButton, ButtonVariant};
use crate::providers::{use_auth_context, AuthAction, AuthMessage};
use crate::router::SneuRoute;

#[function_component(Home)]
pub fn index() -> Html {
    let auth_context = use_auth_context();

    match &*auth_context {
        AuthMessage::NotAuthed => html! {
            <Redirect<SneuRoute> to={SneuRoute::SignIn} />
        },
        AuthMessage::Authed(auth) => html! {
            <div>
                <h4>{ format!("Welcome, {}...!", auth.username) }</h4>
                <BulmaButton
                    variant={ButtonVariant::Warning}
                    onclick={Callback::from(move |_| auth_context.dispatch(AuthAction::SignOut))}
                >
                    { "Sign out" }
                </BulmaButton>
            </div>
        },
    }
}
