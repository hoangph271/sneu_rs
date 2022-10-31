use crate::utils::no_op;
use crate::utils::sneu_api;

use wasm_bindgen_futures::spawn_local;
use yew::{use_state_eq};
use yew_hooks::use_effect_once;

pub fn use_random_image_url() -> Option<String> {
    let image_url = use_state_eq(|| None);

    use_effect_once({
        let image_url = image_url.clone();

        || {
            spawn_local(async move {
                let redirect_url = sneu_api::raw_get("/files/random/raw?mime=image/&preview=true")
                    .await
                    .unwrap();

                image_url.set(Some(redirect_url.url()));
            });

            no_op
        }
    });

    (*image_url).clone()
}
