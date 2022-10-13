use std::cmp::Ordering;

use crate::{
    components::*,
    utils::{no_op, sneu_api::ApiHandler},
};
use hbp_types::{ApiList, Challenge};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct UseLastedProps {}

#[derive(PartialEq, Properties)]
pub struct LastedItemProps {
    challenge: Challenge,
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
            <Wasted note={title.clone()} />
            <Whys why={why.clone()} class="flex-grow" />
            <Lasted
                started_at={started_at.clone()}
                end_at={end_at.clone()}
            />
        </div>
    }
}

#[function_component(UseLasted)]
pub fn use_lasted(props: &UseLastedProps) -> Html {
    let UseLastedProps {} = props;
    let challenges = use_state(|| Option::<Vec<Challenge>>::None);
    let api_hander = use_state_eq(|| ApiHandler { jwt: None });

    use_effect_with_deps(
        {
            let challenges = challenges.clone();
            let api_hander = api_hander.clone();

            move |_| {
                spawn_local(async move {
                    let mut items = api_hander
                        .json_get::<ApiList<Challenge>>("/challenges")
                        .await
                        .unwrap()
                        .items;

                    items.sort_by(|c1, c2| {
                        if c1.finished {
                            Ordering::Greater
                        } else if c2.finished {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    });

                    challenges.set(Some(items));
                });

                no_op
            }
        },
        (*api_hander).clone(),
    );

    with_loader((*challenges).as_ref().map(|c| c.clone()), |challenges| {
        html! {
            <div
                class="w-screen h-screen bg-cover bg-no-repeat bg-center grid md:grid-cols-2 lg:grid-cols-3 sm:grid-cols-1"
                style="font-family: Monocraft, monospace; background-image: url(https://uselasted.netlify.app/static/media/751400.530ccd0e600c0697d435.png)"
            >
                {challenges.iter().map(|challenge| {
                    html! {
                        <LastedItem
                            key={challenge.id.clone()}
                            challenge={challenge.clone()}
                        />
                    }
                }).collect::<Html>()}
            </div>
        }
    })
}
