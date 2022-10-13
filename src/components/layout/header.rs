use crate::router::*;
use chrono::{TimeZone, Utc};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    let sucker = Utc::now()
        .signed_duration_since(Utc.ymd(1997, 3, 5).and_hms(0, 0, 0))
        .num_weeks() as f64
        / 52.0;

    let sucker = format!("Sucker is {sucker:.2} years old");

    html! {
        <header class="p-4 shadow-md hover:shadow-xl">
            <nav class="flex">
                <SneuLink to={SneuRoutes::Home} classes="hover:underline">
                    { "@sneu_rs" }
                </SneuLink>
                <div class="flex-grow text-end">
                    { sucker }
                </div>
            </nav>
        </header>
    }
}
