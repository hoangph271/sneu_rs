use crate::{components::*, utils::expect_target};
use chrono::{DateTime, TimeZone, Utc};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CreateUseLastedProps {}

#[function_component(CreateUseLasted)]
pub fn create_use_lasted(props: &CreateUseLastedProps) -> Html {
    let CreateUseLastedProps {} = props;
    let is_loading = use_state_eq(|| false);
    // let (is_loading, onsubmit, sign_in_error, clear_error) =
    //     use_sign_in_handler((*username).clone(), (*password).clone());
    let title = use_state_eq(String::new);
    let why = use_state_eq(String::new);
    // TODO: Flip display on cards...?
    let note = use_state_eq(String::new);
    let started_at = use_state_eq(Utc::now);
    let end_at = use_state_eq(Utc::now);
    // TODO: Random ID

    html! {
        <form>
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
                value={started_at.to_string()}
                on_value_changed={{
                    let started_at = started_at.clone();
                    Callback::from(move |value: String| {
                        let datetime = DateTime::parse_from_str(&value, "")
                            .unwrap()
                            .naive_utc();
                        started_at.set(Utc.from_utc_datetime(&datetime));
                    })
                }}
            />
            <FormInput
                label="End at:"
                placeholder="When will the challenge end...?"
                input_type={InputType::Datetime}
                value={end_at.to_string()}
                on_value_changed={{
                    let end_at = end_at.clone();
                    Callback::from(move |value: String| {
                        let datetime = DateTime::parse_from_str(&value, "")
                            .unwrap()
                            .naive_utc();
                        end_at.set(Utc.from_utc_datetime(&datetime));
                    })
                }}
            />
            <PillButton
                button_type={ButtonType::Submit}
                is_loading={*is_loading}
            >
                { "Create" }
            </PillButton>
        </form>
    }
}
