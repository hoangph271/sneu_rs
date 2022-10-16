use crate::{
    components::*,
    utils::{expect_target, from_datetime_str, to_datetime_str},
};
use chrono::Utc;
use hbp_types::Challenge;
use nanoid::nanoid;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ChallengeFormProps {
    pub onsubmit: Callback<Challenge>,
    pub is_loading: bool,
    #[prop_or_default]
    pub is_edit: bool,
    #[prop_or_else(|| Challenge {
        id: nanoid!(),
        title: String::new(),
        why: String::new(),
        note: String::new(),
        start_at_ms: Utc::now(),
        end_at_ms: Utc::now(),
        finished: false,
    })]
    pub challenge: Challenge,
}

#[function_component(ChallengeForm)]
pub fn challenge_form(props: &ChallengeFormProps) -> Html {
    let ChallengeFormProps {
        onsubmit,
        is_loading,
        challenge,
        is_edit,
    } = props;

    let title = use_state_eq(|| challenge.title.to_owned());
    let why = use_state_eq(|| challenge.why.to_owned());
    // TODO: Flip display on cards...?
    let note = use_state_eq(|| challenge.note.to_owned());
    let start_at_ms = use_state_eq(|| challenge.start_at_ms.to_owned());
    let end_at_ms = use_state_eq(|| challenge.end_at_ms.to_owned());
    let finished = use_state_eq(|| challenge.finished);

    html! {
        <form
            onsubmit={{
                let challenge = Challenge {
                    id: challenge.id.clone(),
                    title: (*title).clone(),
                    why: (*why).clone(),
                    note: (*note).clone(),
                    start_at_ms: (*start_at_ms).clone(),
                    end_at_ms: (*end_at_ms).clone(),
                    finished: (*finished).clone(),
                };
                let onsubmit = onsubmit.clone();

                move |e: FocusEvent| {
                    e.prevent_default();
                    onsubmit.emit(challenge.clone());
                }
            }}
        >
            <FormInput
                label="Title:"
                placeholder="Title of your challenge...?"
                value={(*title).clone()}
                on_value_changed={{
                    let title = title.clone();
                    Callback::from(move |value| title.set(value))
                }}
            />
            <textarea
                placeholder="Explain why you want to start it...?"
                value={(*why).clone()}
                oninput={{
                    let why = why.clone();
                    Callback::from(move |e: InputEvent| why.set(expect_target::<HtmlTextAreaElement>(e.target()).unwrap().value()))
                }}
            />
            <textarea
                placeholder="Other notes...!"
                value={(*note).clone()}
                oninput={{
                    let note = note.clone();
                    Callback::from(move |e: InputEvent| note.set(expect_target::<HtmlTextAreaElement>(e.target()).unwrap().value()))
                }}
            />
            <FormInput
                label="Start at:"
                placeholder="When will the challenge begin...?"
                input_type={InputType::Datetime}
                value={to_datetime_str(&*start_at_ms)}
                on_value_changed={{
                    let start_at_ms = start_at_ms.clone();

                    Callback::from(move |value: String| {
                        start_at_ms.set(from_datetime_str(&value));
                    })
                }}
            />
            <FormInput
                label="End at:"
                placeholder="When will the challenge end...?"
                input_type={InputType::Datetime}
                value={to_datetime_str(&*end_at_ms)}
                on_value_changed={{
                    let end_at_ms = end_at_ms.clone();

                    Callback::from(move |value: String| {
                        end_at_ms.set(from_datetime_str(&value));
                    })
                }}
            />
            <PillButton
                button_type={ButtonType::Submit}
                is_loading={*is_loading}
            >
                { if *is_edit { "Update" } else { "Create" } }
            </PillButton>
        </form>
    }
}
