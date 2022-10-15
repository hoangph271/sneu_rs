use crate::{components::*, utils::sneu_api::ApiHandler};
use hbp_types::Challenge;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CreateUseLastedProps {}

#[function_component(CreateUseLasted)]
pub fn create_use_lasted(props: &CreateUseLastedProps) -> Html {
    let CreateUseLastedProps {} = props;
    let is_loading = use_state_eq(|| false);

    let onsubmit = Callback::from({
        is_loading.set(true);
        let is_loading = is_loading.clone();

        move |challenge: Challenge| {
            let is_loading = is_loading.clone();
            spawn_local({
                let json = serde_json::to_string(&challenge.clone())
                    .expect("{challenge:?} must be a valid JSON value");

                async move {
                    let _ = ApiHandler::default()
                        .json_post::<Challenge>("/challenges", JsValue::from_str(&json))
                        .await
                        .unwrap_or_else(|e| {
                            log::error!("create_use_lasted failed: {e:?}");
                            panic!();
                        });

                    is_loading.set(false);
                }
            });
        }
    });

    html! {
        <ChallengeForm
            {onsubmit}
            is_loading={*is_loading}
        />
    }
}
