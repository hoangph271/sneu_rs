use crate::hooks::{use_authed_api_hander, with_auth_required};
use crate::utils::{no_op, sneu_api::ApiHandler};
use crate::{components::*, hooks::use_history, router::SneuRoutes};
use hbp_types::{ApiItem, Challenge};
use js_sys::encode_uri_component;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::use_effect_once;
use yew_router::prelude::History;

#[derive(PartialEq, Properties, Eq)]
pub struct EditUseLastedProps {
    pub id: String,
}

fn use_challenge(id: &str) -> UseStateHandle<Option<Challenge>> {
    let challenge = use_state_eq(|| Option::<Challenge>::None);

    use_effect_once({
        let id = (*id).to_owned();
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

#[derive(PartialEq, Properties, Eq)]
pub struct EditFormProps {
    pub id: String,
}

#[function_component(EditForm)]
pub fn edit_form(props: &EditFormProps) -> Html {
    let EditFormProps { id } = props;

    let history = use_history();
    let is_loading = use_state_eq(|| false);
    let challenge = use_challenge(id);
    let api_handler = use_authed_api_hander();
    let is_form_loading = *is_loading || api_handler.is_none();

    with_loader((*challenge).clone(), {
        move |loaded_challenge| {
            html! {
                <>
                    <ChallengeForm
                        onsubmit={{
                            let is_loading = is_loading.clone();
                            let challenge = challenge.clone();
                            let history = history.clone();
                            let api_handler = api_handler.clone();

                            Callback::from(move |item| {
                                let challenge = challenge.clone();
                                let is_loading = is_loading.clone();
                                let history = history.clone();
                                let api_handler = api_handler.clone().unwrap();

                                spawn_local(async move {
                                    is_loading.set(true);

                                    challenge.set(Some(
                                        api_handler
                                            .json_put::<ApiItem<Challenge>>(
                                                "/challenges",
                                                JsValue::from_str(&serde_json::to_string(&item).unwrap()),
                                            )
                                            .await
                                            .unwrap()
                                            .item,
                                    ));

                                    is_loading.set(false);
                                    history.push(SneuRoutes::UseLasted);
                                });
                            })
                        }}
                        challenge={loaded_challenge.clone()}
                        is_edit={true}
                        is_loading={is_form_loading}
                    />
                    <PillButton
                        button_type={ButtonType::Button}
                        disabled={is_form_loading}
                        onclick={{
                            let id = encode_uri_component(&loaded_challenge.id);
                            let history = history.clone();

                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                let id = id.clone();
                                let history = history.clone();

                                spawn_local(async move {
                                    ApiHandler::default()
                                        .json_delete::<ApiItem<()>>(&format!("/challenges/{id}"), JsValue::undefined())
                                        .await
                                        .unwrap();

                                    history.push(SneuRoutes::UseLasted)
                                });
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

#[function_component(EditUseLasted)]
pub fn edit_use_lasted(props: &EditUseLastedProps) -> Html {
    let EditUseLastedProps { id } = props;

    with_auth_required(|| {
        html! {
            <EditForm id={id.clone()} />
        }
    })
}
