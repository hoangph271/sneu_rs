use crate::utils::api_url::with_api_root;

use gloo_net::http::Request;
use gloo_utils::document;
use image;
use std::path::PathBuf;
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
                    let redirect_url =
                        Request::get(&with_api_root("/files/random/raw?mime=image/&preview=true"))
                            .send()
                            .await
                            .unwrap()
                            .url();

                    let reader = Request::get(&redirect_url)
                        .send()
                        .await
                        .unwrap()
                        .binary()
                        .await
                        .unwrap();

                    let reader = std::io::Cursor::new(reader);
                    let mut image = image::io::Reader::new(reader);
                    image.set_format(image::ImageFormat::Gif);

                    log::info!("WTF: {:?}", image.with_guessed_format().unwrap().format());

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
