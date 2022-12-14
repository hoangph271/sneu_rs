mod local_library_link;
pub use local_library_link::*;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct LocalLibraryProps {}

#[cfg(feature = "sneu_tauri")]
#[function_component(LocalLibrary)]
pub fn local_library(props: &LocalLibraryProps) -> Html {
    let LocalLibraryProps {} = props;

    html! {
        <div>

        </div>
    }
}

#[cfg(not(feature = "sneu_tauri"))]
#[function_component(LocalLibrary)]
pub fn local_library(props: &LocalLibraryProps) -> Html {
    let LocalLibraryProps {} = props;

    html! {}
}
