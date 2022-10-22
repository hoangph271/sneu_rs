use hbp_types::Challenge;
use yew::prelude::*;

mod footer;
mod title;
mod whys;

use footer::Footer;
use title::Title;
use whys::Whys;

#[derive(PartialEq, Eq)]
pub struct UseLastedProps {}

#[derive(PartialEq, Properties)]
pub struct ChallengeCardProps {
    pub challenge: Challenge,
}
#[function_component(ChallengeCard)]
pub fn challenge_card(props: &ChallengeCardProps) -> Html {
    let ChallengeCardProps { challenge } = props;
    let Challenge {
        id,
        title,
        why,
        start_at_ms,
        end_at_ms,
        finished,
        ..
    } = challenge;

    let class = format!(
        "flex flex-col text-white p-2 h-96 max-h-screen {}",
        if *finished { "opacity-50" } else { "" }
    );

    html! {
        <div {class}>
            <Title id={id.clone()} title={title.clone()} />
            <Whys why={why.clone()} class="flex-grow" />
            <Footer
                started_at={start_at_ms.clone()}
                end_at={end_at_ms.clone()}
            />
        </div>
    }
}
