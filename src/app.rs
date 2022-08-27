use super::providers::AuthProvider;
use super::router;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AppWithRouter)]
pub fn app_with_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::SneuRoute>
                render={Switch::render(router::switch)}
            />
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
