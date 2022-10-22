use super::super::utils::use_lasted;
use crate::utils::friendly_datetime;
use chrono::{DateTime, Utc};
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
                    {friendly_datetime(started_at)}
                </span>
            }
        </div>
    }
}
