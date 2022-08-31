use super::components::*;
use super::providers::AuthProvider;
use super::router::{switch as sneu_switch, SneuRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AppWithRouter)]
pub fn app_with_router() -> Html {
    html! {
        <BrowserRouter>
            <AppTitle />
            <Switch<SneuRoute> render={Switch::render(sneu_switch)} />
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
