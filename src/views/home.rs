use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::components::MarkdownViewer;
use crate::providers::{use_auth_reducer, AuthAction, AuthContext};
use crate::router::SneuRoute;

#[function_component(Home)]
pub fn index() -> Html {
    let auth_reducer = use_auth_reducer();

    match (*auth_reducer).clone() {
        AuthContext::NotAuthed => html! {
            <Redirect<SneuRoute> to={SneuRoute::SignIn} />
        },
        AuthContext::Authed(auth) => html! {
            <div>
                <h4>{ format!("Welcome, {}...!", auth.username) }</h4>
                <button
                    class="button is-warning"
                    type="button"
                    onclick={move |_| auth_reducer.dispatch(AuthAction::SignOut)}
                >{ "Sign out" }</button>
                <MarkdownViewer />
            </div>
        },
    }
}
