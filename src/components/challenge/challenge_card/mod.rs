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

#[derive(PartialEq, Properties, Eq)]
pub struct ChallengeCardProps {
    pub challenge: Challenge,
}
#[function_component(ChallengeCard)]
pub fn challenge_card(props: &ChallengeCardProps) -> Html {
    let ChallengeCardProps { challenge } = props;
    let is_flipped = use_state_eq(|| false);

    let Challenge {
        id,
        title,
        why,
        start_at_ms,
        end_at_ms,
        finished,
        note,
        ..
    } = challenge;

    let class = format!(
        "flex flex-col text-white p-2 h-96 max-h-screen {}",
        if *finished { "opacity-50" } else { "" }
    );

    let text = if *is_flipped {
        note.clone()
    } else {
        why.clone()
    };

    html! {
        <div
            {class}
            onclick={Callback::from(move |_| { is_flipped.set(!*is_flipped) })}
        >
            <Title id={id.clone()} title={title.clone()} />
            <Whys {text} class="flex-grow" />
            <Footer
                started_at={*start_at_ms}
                end_at={*end_at_ms}
            />
        </div>
    }
}
