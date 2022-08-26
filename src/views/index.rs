use gloo_net::http::{Method, Request};
use httpstatus::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct IndexProps {}

#[derive(Deserialize, Debug)]
struct ApiItem<T: Serialize> {
    #[serde(deserialize_with = "status_code_from_u16")]
    status_code: StatusCode,
    item: T,
}

fn status_code_from_u16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<StatusCode, D::Error> {
    let code: u16 = Deserialize::deserialize(deserializer)?;

    Ok(StatusCode::from(code))
}

fn handle_submit(username: String, password: String) {
    wasm_bindgen_futures::spawn_local(async move {
        #[derive(Serialize)]
        struct LoginPayload {
            username: String,
            password: String,
        }

        let json_payload = serde_json::to_string(&LoginPayload { username, password }).unwrap();
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

        log::info!("{} | {}", api_item.status_code, api_item.item);
    });
}

#[function_component(Index)]
pub fn index(_: &IndexProps) -> Html {
    let username = use_state_eq(|| "".to_owned());
    let password = use_state_eq(|| "".to_owned());

    html! {
        <form
            action="http://localhost:8000/api/v1/users/signin"
            method="post"
            style="display: flex; flex-direction: column; gap: 1rem; align-items: center; justify-content: center;"
            onsubmit={{
                let username = (*username).clone();
                let password = (*password).clone();

                Callback::from(move |e: FocusEvent| {
                    e.prevent_default();

                    handle_submit(username.clone(), password.clone())
                })
            }}
        >
            <label>
                <span>{ "Username:" }</span>
                <input
                    type="text"
                    value={(*username).clone()}
                    oninput={move |e: InputEvent| {
                        let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                        username.set(value);
                    }}
                />
            </label>
            <label>
                <span>{ "Password:" }</span>
                <input
                    type="password"
                    value={(*password).clone()}
                    oninput={move |e: InputEvent| {
                        let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                        password.set(value);
                    }}
                />
            </label>
            <button type="submit">{ "Sign in" }</button>
        </form>
    }
}
