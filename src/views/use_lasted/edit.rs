use crate::components::*;
use hbp_types::{ApiItem, Challenge};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;

use crate::utils::{no_op, sneu_api::ApiHandler};

#[derive(PartialEq, Properties)]
pub struct EditUseLastedProps {
    pub id: String,
}

#[function_component(EditUseLasted)]
pub fn edit_use_lasted(props: &EditUseLastedProps) -> Html {
    let EditUseLastedProps { id } = props;

    let is_loading = use_state_eq(|| false);
    let challenge = use_state_eq(|| Option::<Challenge>::None);

    use_effect_once({
        let id = id.clone();
        let challenge = challenge.clone();

        || {
            spawn_local(async move {
                let api_handler = ApiHandler::default();

                challenge.set(Some(
                    api_handler
                        .json_get::<ApiItem<Challenge>>(&format!("/challenges/{id}"))
                        .await
                        .unwrap()
                        .item,
                ));
            });

            no_op
        }
    });

    with_loader((*challenge).clone(), move |challenge| {
        let onsubmit = Callback::from(|_challenge| {});

        html! {
            <ChallengeForm
                {onsubmit}
                {challenge}
                is_edit={true}
                is_loading={*is_loading}
            />
        }
    })
}
