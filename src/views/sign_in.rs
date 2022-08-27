use gloo_net::http::{Method, Request};
use httpstatus::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;
use yew_router::prelude::Redirect;

use crate::{
    providers::{use_auth_reducer, Auth, AuthAction, AuthContext},
    router::SneuRoute,
};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let auth_reducer = use_auth_reducer();
    log::info!("{:?}", *auth_reducer);

    let username = use_state_eq(|| "".to_owned());
    let password = use_state_eq(|| "".to_owned());
    let is_loading = use_bool_toggle(false);

    if let AuthContext::Authed(_) = *auth_reducer {
        return html! { <Redirect<SneuRoute> to={SneuRoute::Home} />};
    }

    html! {
        <form
            action="http://localhost:8000/api/v1/users/signin"
            method="post"
            style="display: flex; flex-direction: column; gap: 1rem; align-items: center; justify-content: center;"
            onsubmit={{
                let username = (*username).clone();
                let password = (*password).clone();
                let is_loading = is_loading.clone();

                Callback::from(move |e: FocusEvent| {
                    e.prevent_default();

                    let auth_reducer = auth_reducer.clone();
                    let is_loading = is_loading.clone();

                    is_loading.toggle();

                    handle_submit(username.clone(), password.clone(), move |auth| {
                        is_loading.toggle();
                        auth_reducer.dispatch(AuthAction::SignIn(auth));
                    });
                })
            }}
        >
            <div class="field">
                <label class="label">{ "Username:" }</label>
                <div class="control has-icons-left has-icons-right">
                    <input
                        type="text"
                        class="input is-success"
                        placeholder="Your username"
                        value={(*username).clone()}
                        oninput={move |e: InputEvent| {
                            let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                            username.set(value);
                        }}
                    />
                    <span class="icon is-small is-left">
                        <i class="fas fa-user"></i>
                    </span>
                </div>
            </div>
            <div class="field">
                <label class="label">{ "Password:" }</label>
                <div class="control has-icons-left has-icons-right">
                    <input
                        type="password"
                        class="input is-success"
                        placeholder="Your password"
                        value={(*password).clone()}
                        oninput={move |e: InputEvent| {
                            let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                            password.set(value);
                        }}
                    />
                    <span class="icon is-small is-left">
                        <i class="fas fa-key"></i>
                    </span>
                </div>
            </div>
            <button
                type="submit"
                disabled={*is_loading}
                class="button is-primary"
            >{ "Sign in" }</button>
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
