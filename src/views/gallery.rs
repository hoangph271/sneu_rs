use std::path::PathBuf;

use gloo_net::http::Request;
use gloo_utils::document;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::no_op;

fn use_image_url() -> Option<String> {
    let image_url = use_state_eq(|| None);

    use_effect_with_deps(
        {
            let image_url = image_url.clone();

            |_| {
                spawn_local(async move {
                    let redirect_url = Request::get(
                        "https://alpha-sneu.xyz/api/v1/files/random/raw?mime=image/&preview=true",
                    )
                    .send()
                    .await
                    .unwrap()
                    .url();

                    image_url.set(Some(redirect_url));
                });

                no_op
            }
        },
        (),
    );

    (*image_url).clone()
}

#[function_component(Gallery)]
pub fn gallery() -> Html {
    let image_url = use_image_url();

    use_effect_with_deps(
        {
            let title = image_url.clone().map(|image_url| {
                PathBuf::from(image_url)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });

            move |_| {
                if let Some(title) = title {
                    document().set_title(&title);
                }

                no_op
            }
        },
        image_url.clone(),
    );

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
