use crate::utils::expect_target;

use nanoid::nanoid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Password,
}

impl InputType {
    fn to_type_attr(&self) -> String {
        match self {
            InputType::Text => "text".to_owned(),
            InputType::Password => "password".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq, Default)]
pub struct FormInputProps {
    #[prop_or_else(|| nanoid!())]
    pub id: String,
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
    #[prop_or_default]
    pub label: String,
    pub value: String,
    #[prop_or_default]
    pub fa_icon: String,
    pub on_value_changed: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(FormInput)]
pub fn form_input(props: &FormInputProps) -> Html {
    let FormInputProps {
        id,
        label,
        on_value_changed,
        input_type,
        placeholder,
        value,
        fa_icon,
        ..
    } = props;

    let oninput = {
        let on_value_changed = on_value_changed.clone();

        move |e: InputEvent| {
            let value = expect_target::<HtmlInputElement>(e.target())
                .expect("form_input event target casting failed")
                .value();

            on_value_changed.emit(value);
        }
    };

    html! {
        <div>
            if !label.is_empty() {
                <label class="label" for={id.clone()}>{ label.clone() }</label>
            }
            <div
                class="border rounded border-gray-200 bg-slate-300 shadow-sm shadow-gray-200 focus-within:shadow-sky-500 focus-within:border-sky-200"
            >
                <input
                    id={id.clone()}
                    type={input_type.to_type_attr()}
                    class="px-1.5 py-1 rounded-l outline-0"
                    placeholder={placeholder.to_owned()}
                    value={value.clone()}
                    {oninput}
                />
                if !fa_icon.is_empty() {
                    <span class="px-2 rounded-r">
                        <i class={format!("fas {}", fa_icon)}></i>
                    </span>
                }
            </div>
        </div>
    }
}
