use wasm_bindgen::JsCast;
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
    #[prop_or(InputType::Text)]
    pub input_type: InputType,
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
    let on_value_changed = props.on_value_changed.clone();

    html! {
        <div class="field">
            if !props.label.is_empty() {
                <label class="label">{ props.label.clone() }</label>
            }
            <div class="control has-icons-left has-icons-right">
                <input
                    type={props.input_type.to_type_attr()}
                    class="input"
                    placeholder={props.placeholder.to_owned()}
                    value={props.value.clone()}
                    oninput={move |e: InputEvent| {
                        let value = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value();

                        on_value_changed.emit(value);
                    }}
                />

                if !props.fa_icon.is_empty() {
                    <span class="icon is-small is-left">
                        <i class={format!("fas {}", props.fa_icon)}></i>
                    </span>
                }
            </div>
        </div>
    }
}
