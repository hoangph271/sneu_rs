use super::utils::use_lasted;
use chrono::{DateTime, Utc};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LastedProps {
    pub started_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[function_component(Lasted)]
pub fn lasted(props: &LastedProps) -> Html {
    let LastedProps { started_at, end_at } = props;
    let (lasted, progress, is_done) = use_lasted(started_at, end_at);

    html! {
        <div class="bg-gray-900/75 p-4 text-center rounded-b">
            <span>{ "Lasted " }</span>
            <span style="font-style: italic">
                {if is_done {
                    format!("{lasted} [{progress}%]...!")
                } else {
                    format!("{lasted}...!")
                }}
            </span>
        </div>
    }
}
