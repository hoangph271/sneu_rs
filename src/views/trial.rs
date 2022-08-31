use crate::components::*;
use yew::prelude::*;

#[function_component(Trial)]
pub fn trial() -> Html {
    html! {
        <CenteredViewport>
            <Toast
                header="Dude"
            />
            // { "TODO: // Add something to use SneuRoute::Trial" }
        </CenteredViewport>
    }
}
