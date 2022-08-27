use yew::prelude::*;

use super::{AuthContext, AuthMessage};

#[derive(PartialEq, Properties)]
pub struct AuthProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth = use_reducer(AuthMessage::default);

    html! {
        <ContextProvider<AuthContext> context={auth}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}
