mod title;
mod utils;
mod whys;

pub use title::*;
pub use whys::*;

use chrono::{DateTime, Utc};
use yew::prelude::*;

use self::utils::use_lasted;

#[derive(PartialEq, Properties)]
pub struct FooterProps {
    pub started_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let FooterProps { started_at, end_at } = props;
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
