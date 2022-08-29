use crate::{
    providers::{use_auth_context, AuthAction, AuthMessage, AuthPayload},
    utils::{
        json,
        sneu_api::{self, ApiItem, ApiResult},
    },
};

use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::FocusEvent;
use yew::Callback;
use yew_hooks::use_bool_toggle;

pub fn use_sign_in_handler(username: String, password: String) -> (bool, Callback<FocusEvent>) {
    let auth_context = use_auth_context();
    let is_loading = use_bool_toggle(false);

    let onsubmit = {
        let is_loading = is_loading.clone();

        Callback::from({
            let auth_context = auth_context.clone();

            move |e: FocusEvent| {
                e.prevent_default();

                let is_loading = is_loading.clone();
                let auth_context = auth_context.clone();

                is_loading.toggle();

                handle_submit(username.clone(), password.clone(), move |auth_result| {
                    is_loading.toggle();

                    match auth_result {
                        Ok(auth) => {
                            AuthMessage::Authed(auth.clone()).persist_locally();
                            auth_context.dispatch(AuthAction::SignIn(auth))
                        }
                        Err(e) => {
                            log::error!("handle_submit() failed: {e:?}")
                        }
                    };
                });
            }
        })
    };

    (*is_loading, onsubmit)
}

fn handle_submit<OnAuth>(username: String, password: String, on_authed: OnAuth)
where
    OnAuth: Fn(ApiResult<AuthPayload>) + 'static,
{
    spawn_local(async move {
        #[derive(Serialize)]
        struct LoginPayload {
            username: String,
            password: String,
        }

        let sign_in_payload = json::stringify(&LoginPayload {
            username: username.clone(),
            password,
        });

        on_authed(
            sneu_api::post::<ApiItem<String>>("/users/signin", JsValue::from_str(&sign_in_payload))
                .await
                .map(|api_item| AuthPayload {
                    username,
                    jwt: api_item.item,
                }),
        );
    });
}
