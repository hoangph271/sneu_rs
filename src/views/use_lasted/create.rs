use crate::{
    components::*,
    hooks::{use_authed_api_hander, use_history},
    router::SneuRoutes,
};
use hbp_types::Challenge;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::History;

#[derive(PartialEq, Properties, Eq)]
pub struct CreateUseLastedProps {}

#[function_component(CreateUseLasted)]
pub fn create_use_lasted(props: &CreateUseLastedProps) -> Html {
    let CreateUseLastedProps {} = props;
    let history = use_history();
    let is_loading = use_state_eq(|| false);
    let api_handler = use_authed_api_hander().unwrap();

    let onsubmit = Callback::from({
        let is_loading = is_loading.clone();

        move |challenge: Challenge| {
            let is_loading = is_loading.clone();
            let history = history.clone();
            let api_handler = api_handler.clone();

            spawn_local({
                let json = serde_json::to_string(&challenge)
                    .expect("{challenge:?} must be a valid JSON value");

                async move {
                    is_loading.set(true);

                    let _ = api_handler
                        .json_post::<Challenge>("/challenges", JsValue::from_str(&json))
                        .await
                        .unwrap_or_else(|e| {
                            log::error!("create/edit challenge failed: {e:?}");
                            panic!();
                        });

                    is_loading.set(false);
                    history.push(SneuRoutes::UseLasted);
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
