use super::providers::AuthProvider;
use super::router::{switch as sneu_switch, SneuRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AppWithRouter)]
pub fn app_with_router() -> Html {
    html! {
        <BrowserRouter>
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

#[cfg(test)]
mod test {
    use super::AppWithContext;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn app_runs() {
        yew::start_app::<AppWithContext>();
    }
}
