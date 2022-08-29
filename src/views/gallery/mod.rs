mod use_random_image_url;
mod use_synced_document_title;
use use_random_image_url::use_random_image_url;
use use_synced_document_title::use_synced_document_title;

use std::path::PathBuf;
use yew::prelude::*;

fn title_from_url(url: String) -> String {
    PathBuf::from(url)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

#[function_component(Gallery)]
pub fn gallery() -> Html {
    let image_url = use_random_image_url();

    use_synced_document_title(image_url.clone().map(title_from_url));

    html! {
        if let Some (image_url) = image_url {
            <img
                src={ image_url.clone() }
                style="max-width: 80%; display: block; margin: auto; }"
            />
        } else {
            <div>{ "...!" }</div>
        }
    }
}
