use yew::prelude::*;
use yew_router::prelude::Redirect;

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
                <button
                    class="button is-warning"
                    type="button"
                    onclick={move |_| {
                        auth_context.dispatch(AuthAction::SignOut);
                    }}
                >{ "Sign out" }</button>
            </div>
        },
    }
}
