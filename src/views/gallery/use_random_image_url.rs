use crate::utils::no_op;
use crate::utils::sneu_api;

use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state_eq};

pub fn use_random_image_url() -> Option<String> {
    let image_url = use_state_eq(|| None);

    use_effect_with_deps(
        {
            let image_url = image_url.clone();

            |_| {
                spawn_local(async move {
                    let redirect_url =
                        sneu_api::raw_get("/files/random/raw?mime=image/&preview=true")
                            .await
                            .unwrap();

                    image_url.set(Some(redirect_url.url()));
                });

                no_op
            }
        },
        image_url.clone(),
    );

    (*image_url).clone()
}
