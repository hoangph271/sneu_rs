use crate::router::{SneuLink, SneuRoute};
use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="card">
            <header class="card-header">
                <p class="card-header-title is-justify-content-center">
                    <h4>{ "404 | Not Found" }</h4>
                </p>
            </header>
            <div class="card-content">
                <div class="content has-text-centered">
                    <SneuLink to={SneuRoute::Home}>
                        { "Click here to get home" }
                    </SneuLink>
                </div>
            </div>
        </div>
    }
}
