use super::providers::AuthContext;
use super::router;
use log::info;
use yew::{function_component, html, use_reducer, ContextProvider};
use yew_router::prelude::*;

#[function_component(Router)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::SneuRoute>
                render={Switch::render(router::switch)}
            />
        </BrowserRouter>
    }
}

#[function_component(App)]
pub fn app_with_context() -> Html {
    let auth_reducer = use_reducer(AuthContext::default);
    info!("{:?}", auth_reducer);

    html! {
        <ContextProvider<AuthContext> context={(*auth_reducer).clone()}>
            <Router />
        </ContextProvider<AuthContext>>
    }
}
