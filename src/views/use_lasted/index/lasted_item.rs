use crate::components::*;
use hbp_types::Challenge;
use yew::prelude::*;

#[derive(PartialEq, Eq)]
pub struct UseLastedProps {}

#[derive(PartialEq, Properties)]
pub struct LastedItemProps {
    pub challenge: Challenge,
}
#[function_component(LastedItem)]
pub fn lasted_item(props: &LastedItemProps) -> Html {
    let LastedItemProps { challenge } = props;
    let Challenge {
        title,
        why,
        started_at,
        end_at,
        finished,
        ..
    } = challenge;

    let class = format!(
        "max-w-fit m-auto flex flex-col text-white p-2 h-96 max-h-screen {}",
        if *finished { "opacity-50" } else { "" }
    );

    html! {
        <div {class}>
            <Title title={title.clone()} />
            <Whys why={why.clone()} class="flex-grow" />
            <Footer
                started_at={started_at.clone()}
                end_at={end_at.clone()}
            />
        </div>
    }
}
