use super::utils::use_lasted;
use chrono::{DateTime, Utc};
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct FooterProps {
    pub started_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps { started_at, end_at } = props;
    let (lasted, progress, is_done, is_started) = use_lasted(started_at, end_at);
    let started_at = js_sys::Date::new(&JsValue::from(started_at.to_string())).to_date_string();

    html! {
        <div class="bg-gray-900/75 p-4 text-center rounded-b">
            if is_started {
                <span>{ "Lasted " }</span>
                <span style="font-style: italic">
                    {if is_done {
                        format!("{lasted} [{progress}%]...!")
                    } else {
                        format!("{lasted}...!")
                    }}
                </span>
            } else {
                <span>{ "Start at: " }</span>
                <span style="font-style: italic">
                    {started_at}
                </span>
            }
        </div>
    }
}
