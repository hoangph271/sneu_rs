use crate::{
    components::ButtonType,
    providers::{use_auth_context, AuthAction},
    utils::no_op,
};
use gloo_net::http::{Method, Request};
use httpstatus::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;
use yew_router::prelude::{use_history, History};

use crate::{
    components::{BulmaButton, FormInput, InputType},
    providers::{Auth, AuthMessage},
    router::SneuRoute,
};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let history = use_history().unwrap();
    let auth_context = use_auth_context();

    let username = use_state_eq(|| "".to_owned());
    let password = use_state_eq(|| "".to_owned());
    let is_loading = use_bool_toggle(false);

    use_effect_with_deps(
        {
            let auth = (*auth_context).clone();

            move |_| {
                if let AuthMessage::Authed(_) = auth {
                    history.push(SneuRoute::Home)
                }

                no_op
            }
        },
        auth_context.clone(),
    );

    html! {
        <form
            method="post"
            action="http://localhost:8000/api/v1/users/signin"
            style="height: 100vh;"
            class="container is-fluid is-flex is-flex-direction-column is-justify-content-center"
            onsubmit={{
                let username = (*username).clone();
                let password = (*password).clone();
                let is_loading = is_loading.clone();

                Callback::from({
                    let auth_context = auth_context.clone();

                    move |e: FocusEvent| {
                        e.prevent_default();

                        let is_loading = is_loading.clone();
                        let auth_context = auth_context.clone();

                        is_loading.toggle();

                        handle_submit(username.clone(), password.clone(), move |auth| {
                            is_loading.toggle();
                            auth_context.dispatch(AuthAction::SignIn(auth))
                        });
                    }

                })
            }}
        >
            <FormInput
                fa_icon="fa-user"
                label="Username:"
                placeholder="Your username...?"
                value={(*username).clone()}
                on_value_changed={{
                    let username = username.clone();
                    Callback::from(move |value| username.set(value))
                }}
            />
            <FormInput
                fa_icon="fa-key"
                label="Password:"
                placeholder="Your password...?"
                input_type={InputType::Password}
                value={(*password).clone()}
                on_value_changed={{
                    let password = password.clone();
                    Callback::from(move |value| password.set(value))
                }}
            />
            <BulmaButton
                button_type={ButtonType::Submit}
                disabled={*is_loading}
            >
                { "Sign in" }
            </BulmaButton>
        </form>
    }
}

#[derive(Deserialize, Debug)]
struct ApiItem<T: Serialize> {
    #[serde(deserialize_with = "status_code_from_u16")]
    #[allow(dead_code)]
    status_code: StatusCode,
    item: T,
}

fn status_code_from_u16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<StatusCode, D::Error> {
    let code: u16 = Deserialize::deserialize(deserializer)?;

    Ok(StatusCode::from(code))
}

fn handle_submit<OnAuth>(username: String, password: String, on_authed: OnAuth)
where
    OnAuth: Fn(Auth) + 'static,
{
    wasm_bindgen_futures::spawn_local(async move {
        #[derive(Serialize)]
        struct LoginPayload {
            username: String,
            password: String,
        }

        let json_payload = serde_json::to_string(&LoginPayload {
            username: username.clone(),
            password,
        })
        .unwrap();
        let login_payload = JsValue::from_str(&json_payload);

        let api_item: ApiItem<String> = Request::post("https://alpha-sneu.xyz/api/v1/users/signin")
            .method(Method::POST)
            .body(login_payload)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        on_authed(Auth {
            username: username.clone(),
            jwt: api_item.item,
        });
    });
}
