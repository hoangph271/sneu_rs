use crate::{
    providers::{use_expected_auth_context, AuthAction, AuthMessage, AuthPayload},
    utils::{
        json,
        sneu_api::{ApiHandler, ApiResult},
    },
};

use hbp_types::ApiItem;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::FocusEvent;
use yew::{use_state_eq, Callback};
use yew_hooks::use_bool_toggle;

pub fn use_sign_in_handler(
    username: &String,
    password: &String,
) -> (bool, Callback<FocusEvent>, String, Callback<()>) {
    let auth_context = use_expected_auth_context();
    let is_loading = use_bool_toggle(false);
    let sign_in_error = use_state_eq(String::new);

    let onsubmit = {
        let is_loading = is_loading.clone();
        let sign_in_error = sign_in_error.clone();

        Callback::from({
            let username = (*username).clone();
            let password = (*password).clone();

            move |e: FocusEvent| {
                e.prevent_default();

                let is_loading = is_loading.clone();
                let sign_in_error = sign_in_error.clone();
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
                            log::error!("handle_submit() failed: {e:?}");
                            sign_in_error.set(e.to_string());
                        }
                    };
                });
            }
        })
    };

    let clear_error = Callback::from({
        let sign_in_error = sign_in_error.clone();

        move |_| sign_in_error.set(String::new())
    });

    (*is_loading, onsubmit, (*sign_in_error).clone(), clear_error)
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
            ApiHandler::default()
                .json_post::<ApiItem<String>>("/users/signin", JsValue::from_str(&sign_in_payload))
                .await
                .map(|api_item| AuthPayload {
                    username,
                    jwt: api_item.item,
                }),
        );
    });
}
