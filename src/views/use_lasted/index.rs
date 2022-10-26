use crate::{
    components::{layout::Header, *},
    utils::{no_op, sneu_api::ApiHandler},
};
use hbp_types::{ApiList, Challenge};
use std::cmp::Ordering;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct UseLastedProps {}

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
                            c1.start_at_ms.cmp(&c2.start_at_ms)
                        }
                    });

                    challenges.set(Some(items));
                });

                no_op
            }
        },
        (*api_hander).clone(),
    );

    html! {
        <div
            class="h-screen text-lg overflow-auto"
            style="font-family: monospace;"
        >
            <Header />
            {with_loader((*challenges).as_ref().map(|c| c.clone()), |challenges| {
                html! {
                    <div class="w-screen grid md:grid-cols-2 lg:grid-cols-3 sm:grid-cols-1">
                        {challenges.iter().map(|challenge| {
                            html! {
                                <ChallengeCard
                                    key={challenge.id.clone()}
                                    challenge={challenge.clone()}
                                />
                            }
                        }).collect::<Html>()}
                    </div>
                }
            })}
        </div>
    }
}
