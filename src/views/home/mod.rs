use crate::router::SneuRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Redirect<SneuRoutes> to={SneuRoutes::UseLasted} />
    }
}
