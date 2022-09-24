use crate::router::{SneuLink, SneuRoutes};
use yew::prelude::*;

#[function_component(LocalLibraryLink)]
pub fn local_library_link() -> Html {
    if crate::utils::is_tauri_app() {
        html! {
            <SneuLink to={SneuRoutes::LocalLibrary}>
                { "Local Library" }
            </SneuLink>
        }
    } else {
        html! {}
    }
}
