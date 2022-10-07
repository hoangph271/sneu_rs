use crate::components::layout::Header;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Header />
        </>
    }
}
