use super::components::AppTitle;
use super::providers::AuthProvider;
use super::router::switch as sneu_switch;
use crate::router::SneuRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AppWithRouter)]
pub fn app_with_router() -> Html {
    html! {
        <BrowserRouter>
            <AppTitle />
            <Switch<SneuRoutes> render={Switch::render(sneu_switch)} />
        </BrowserRouter>
    }
}

#[function_component(AppWithContext)]
pub fn app_with_context() -> Html {
    html! {
        <AuthProvider>
            <AppWithRouter />
        </AuthProvider>
    }
}
