use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::providers::AuthAction;
use crate::providers::AuthContext;
use crate::router::SneuRoute;

#[function_component(Home)]
pub fn index() -> Html {
    let auth_context = use_context::<AuthContext>().unwrap();
    let auth_reducer = use_reducer_eq(|| auth_context);

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
            </div>
        },
    }
}
