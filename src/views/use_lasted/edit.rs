use crate::{components::*, hooks::use_history, router::SneuRoutes};
use hbp_types::{ApiItem, Challenge};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::History;

use crate::utils::{no_op, sneu_api::ApiHandler};

#[derive(PartialEq, Properties)]
pub struct EditUseLastedProps {
    pub id: String,
}

fn use_challenge(id: &str) -> UseStateHandle<Option<Challenge>> {
    let challenge = use_state_eq(|| Option::<Challenge>::None);

    use_effect_once({
        let id = id.clone().to_owned();
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

    challenge
}

#[function_component(EditUseLasted)]
pub fn edit_use_lasted(props: &EditUseLastedProps) -> Html {
    let EditUseLastedProps { id } = props;

    let history = use_history();
    let is_loading = use_state_eq(|| false);
    let challenge = use_challenge(id);

    with_loader((*challenge).clone(), {
        move |loaded_challenge| {
            html! {
                <>
                    <ChallengeForm
                        onsubmit={{
                            let is_loading = is_loading.clone();
                            let challenge = challenge.clone();
                            let history = history.clone();

                            Callback::from(move |item| {
                                is_loading.set(true);
                                let challenge = challenge.clone();

                                spawn_local(async move {
                                    challenge.set(Some(
                                        ApiHandler::default()
                                            .json_put::<ApiItem<Challenge>>(
                                                "/challenges",
                                                JsValue::from_str(&serde_json::to_string(&item).unwrap()),
                                            )
                                            .await
                                            .unwrap()
                                            .item,
                                    ));
                                });

                                is_loading.set(false);
                                history.push(SneuRoutes::UseLasted);
                            })
                        }}
                        challenge={loaded_challenge.clone()}
                        is_edit={true}
                        is_loading={*is_loading}
                    />
                    <PillButton
                        button_type={ButtonType::Button}
                        onclick={{
                            let id = loaded_challenge.id.clone();
                            let history = history.clone();

                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                let id = id.clone();

                                spawn_local(async move {
                                    ApiHandler::default()
                                    .json_delete::<ApiItem<()>>(&format!("/challenges/{id}"), JsValue::undefined())
                                    .await
                                    .unwrap()
                                    .item;
                                });

                                history.push(SneuRoutes::UseLasted)
                            })
                        }}
                    >
                        { "Delete" }
                    </PillButton>
                </>
            }
        }
    })
}
