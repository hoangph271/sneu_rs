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
    let (lasted, progress) = use_lasted(started_at, end_at);

    html! {
        <div class="bg-gray-900/75 py-4 text-center rounded-t">
            <span>{ "You lasted " }</span>
            <span style="font-style: italic">
                { format!("{lasted} [{progress}%]...!") }
            </span>
        </div>
    }
}
