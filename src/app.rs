use super::{providers::AuthContext, views::Index};
use yew::{function_component, html, use_reducer, ContextProvider};

#[function_component(App)]
pub fn app() -> Html {
    let auth_reducer = use_reducer(AuthContext::default);

    html! {
        <ContextProvider<AuthContext> context={(*auth_reducer).clone()}>
            <Index />
        </ContextProvider<AuthContext>>
    }
}
