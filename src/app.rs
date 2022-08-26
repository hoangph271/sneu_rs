use super::{providers::AuthContext, views::Index};
use yew::{function_component, html, use_state, ContextProvider};

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(AuthContext::default);

    html! {
        <ContextProvider<AuthContext> context={(*ctx).clone()}>
            <Index />
        </ContextProvider<AuthContext>>
    }
}
